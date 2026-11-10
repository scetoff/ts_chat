use anyhow::{Context, Result};
use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};
use std::{io::BufReader, num::NonZeroUsize, time::Duration};
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

        let metadata_reader = IcyMetadataReader::new(
            reader,
            icy_headers.metadata_interval(),
            |_metadata| {},
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
        player.append(decoder);
        player.play();

        Ok(Self { _output: output, player })
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
