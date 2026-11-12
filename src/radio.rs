use anyhow::{Context, Result};
use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::{io::BufReader, num::NonZeroUsize, sync::{Arc, Mutex, atomic::{AtomicU32, Ordering}}, time::Duration};
use stream_download::{
    http::{reqwest::Client, HttpStream},
    source::DecodeError,
    storage::{adaptive::AdaptiveStorageProvider, temp::TempStorageProvider},
    Settings, StreamDownload,
};

/// Infer the decoder hint from a stream URL. HostingRadio's extensionless
/// ep128 endpoint is documented as a 128 kbps MP3 stream, so MP3 is the safe
/// fallback for extensionless internet-radio URLs.
fn mime_hint(url: &url::Url) -> &'static str {
    let path = url.path().to_ascii_lowercase();
    if path.ends_with(".aac") || path.ends_with(".aacp") {
        "audio/aac"
    } else if path.ends_with(".ogg") || path.ends_with(".oga") || path.ends_with(".opus") {
        "audio/ogg"
    } else if path.ends_with(".wav") {
        "audio/vnd.wav"
    } else if path.ends_with(".flac") {
        "audio/flac"
    } else {
        "audio/mpeg"
    }
}

/// Keeps both the operating-system audio device and player alive while a radio stream plays.
pub struct RadioPlayback {
    _output: MixerDeviceSink,
    player: Player,
    level: Arc<AtomicU32>,
    track_title: Arc<Mutex<String>>,
}

struct MeteredSource<S> {
    source: S,
    level: Arc<AtomicU32>,
    sum_squares: f64,
    samples: u32,
}

impl<S> Iterator for MeteredSource<S>
where
    S: Source<Item = f32>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.source.next()?;
        self.sum_squares += f64::from(sample) * f64::from(sample);
        self.samples += 1;
        if self.samples >= 1024 {
            let rms = (self.sum_squares / f64::from(self.samples)).sqrt() as f32;
            self.level.store(rms.to_bits(), Ordering::Relaxed);
            self.sum_squares = 0.0;
            self.samples = 0;
        }
        Some(sample)
    }
}

impl<S> Source for MeteredSource<S>
where
    S: Source<Item = f32>,
{
    fn current_span_len(&self) -> Option<usize> { self.source.current_span_len() }
    fn channels(&self) -> rodio::ChannelCount { self.source.channels() }
    fn sample_rate(&self) -> rodio::SampleRate { self.source.sample_rate() }
    fn total_duration(&self) -> Option<Duration> { self.source.total_duration() }
}

impl RadioPlayback {
    pub async fn connect_and_play(url: &str, volume: f32) -> Result<Self> {
        let parsed = url::Url::parse(url).context("Неверный URL радиопотока")?;
        if parsed.scheme() != "http" && parsed.scheme() != "https" {
            anyhow::bail!("Радиопоток должен использовать http:// или https://");
        }
        let mime_type = mime_hint(&parsed);

        // Icecast/Shoutcast stations may insert ICY metadata blocks into the
        // audio byte stream. Request the metadata interval and strip those
        // blocks before passing MP3/AAC bytes to Rodio.
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .user_agent("TSchat/0.1 Internet Radio")
            .request_icy_metadata()
            .build()
            .context("Не удалось создать HTTP-клиент радиопотока")?;

        let stream = tokio::time::timeout(
            Duration::from_secs(15),
            HttpStream::new(client, parsed),
        )
        .await
        .context("Тайм-аут: сервер радиостанции не ответил за 15 секунд")?
        .context("Не удалось установить HTTP-соединение с радиостанцией")?;

        let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
        // Prefetch five seconds of audio, matching the ICY integration's
        // recommended startup buffer calculation.
        let bitrate_kbps = icy_headers.bitrate().unwrap_or(128) as u64;
        let prefetch_bytes = (bitrate_kbps.saturating_mul(1024).saturating_mul(5) / 8)
            .clamp(32 * 1024, 256 * 1024);

        let reader_result = tokio::time::timeout(
            Duration::from_secs(15),
            StreamDownload::from_stream(
                stream,
                AdaptiveStorageProvider::new(
                    TempStorageProvider::default(),
                    NonZeroUsize::new(1024 * 1024).expect("non-zero radio buffer"),
                ),
                Settings::default().prefetch_bytes(prefetch_bytes),
            ),
        )
        .await
        .context("Тайм-аут: радиостанция не прислала аудиоданные за 15 секунд")?;

        let reader = match reader_result {
            Ok(reader) => reader,
            Err(error) => {
                let details = error.decode_error().await;
                return Err(anyhow::anyhow!(
                    "Сервер отклонил или оборвал поток: {details}"
                ));
            }
        };

        let track_title = Arc::new(Mutex::new(String::new()));
        let metadata_target = track_title.clone();
        let metadata_reader = IcyMetadataReader::new(
            reader,
            icy_headers.metadata_interval(),
            move |metadata| {
                if let Ok(metadata) = metadata {
                    if let Some(title) = metadata.stream_title().filter(|title| !title.trim().is_empty()) {
                        if let Ok(mut current) = metadata_target.lock() {
                            *current = title.to_string();
                        }
                    }
                }
            },
        );

        // Decoder::try_from performs synchronous reads. Run it on Tokio's
        // blocking pool so a slow station cannot stall the async runtime.
        let decoder = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::task::spawn_blocking(move || {
                Decoder::builder()
                    .with_data(BufReader::new(metadata_reader))
                    .with_seekable(false)
                    .with_mime_type(&mime_type)
                    .build()
            }),
        )
        .await
        .context("Тайм-аут: аудиоформат станции не удалось определить за 10 секунд")?
        .context("Потоковый декодер завершился с ошибкой")?
        .context("Не удалось декодировать аудиопоток. Проверьте, что URL ведёт непосредственно на MP3/AAC/OGG поток")?;

        // Windows audio-device enumeration/opening can block inside the host
        // audio API. Keep it off the runtime worker and always resolve the UI's
        // "Связь…" state with an actionable timeout/error.
        let output = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::task::spawn_blocking(DeviceSinkBuilder::open_default_sink),
        )
        .await
        .context("Тайм-аут: устройство вывода звука не открылось за 10 секунд")?
        .context("Не удалось выполнить открытие аудиоустройства в фоновом потоке")?
        .context("Не удалось открыть устройство вывода звука Windows")?;
        let player = Player::connect_new(output.mixer());
        player.set_volume(volume.clamp(0.0, 1.0));
        let level = Arc::new(AtomicU32::new(0.0_f32.to_bits()));
        player.append(MeteredSource {
            source: decoder,
            level: level.clone(),
            sum_squares: 0.0,
            samples: 0,
        });
        player.play();

        Ok(Self { _output: output, player, level, track_title })
    }

    pub fn set_volume(&self, volume: f32) {
        self.player.set_volume(volume.clamp(0.0, 1.0));
    }

    pub fn toggle_pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn level(&self) -> f32 {
        f32::from_bits(self.level.load(Ordering::Relaxed)).clamp(0.0, 1.0)
    }

    pub fn track_title(&self) -> Option<String> {
        self.track_title.lock().ok().map(|title| title.clone()).filter(|title| !title.is_empty())
    }

    pub fn stop(&self) {
        self.player.stop();
    }
}


#[cfg(test)]
mod tests {
    use super::mime_hint;

    #[test]
    fn recognizes_common_radio_stream_formats() {
        assert_eq!(
            mime_hint(&url::Url::parse("https://radio.example/live.aac").unwrap()),
            "audio/aac"
        );
        assert_eq!(
            mime_hint(&url::Url::parse("https://radio.example/live.ogg").unwrap()),
            "audio/ogg"
        );
        assert_eq!(
            mime_hint(&url::Url::parse("https://radio.example/live.wav").unwrap()),
            "audio/vnd.wav"
        );
        assert_eq!(
            mime_hint(&url::Url::parse("https://europaplus.hostingradio.ru:8030/ep128").unwrap()),
            "audio/mpeg"
        );
    }
}
