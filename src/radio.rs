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

/// Prefer the actual HTTP response Content-Type (after redirects), then fall
/// back to the URL suffix. Extensionless HostingRadio endpoints commonly use
/// MP3, so audio/mpeg remains the last-resort default.
fn mime_hint(url: &url::Url, content_type: Option<&str>) -> Result<&'static str> {
    let response_type = content_type
        .and_then(|value| value.split(';').next())
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();

    match response_type.as_str() {
        "audio/mpeg" | "audio/mp3" | "audio/x-mpeg" => return Ok("audio/mpeg"),
        "audio/aac" | "audio/aacp" | "audio/x-aac" => return Ok("audio/aac"),
        "audio/ogg" | "application/ogg" | "audio/opus" => return Ok("audio/ogg"),
        "audio/wav" | "audio/wave" | "audio/x-wav" | "audio/vnd.wav" => {
            return Ok("audio/vnd.wav");
        }
        "audio/flac" | "audio/x-flac" => return Ok("audio/flac"),
        "application/vnd.apple.mpegurl"
        | "application/x-mpegurl"
        | "audio/mpegurl"
        | "audio/x-mpegurl"
        | "application/x-scpls"
        | "audio/x-scpls" => {
            anyhow::bail!(
                "Ссылка ведёт на плейлист, а не на аудиопоток. Укажите прямой HTTP(S)-адрес MP3/AAC/OGG."
            );
        }
        "text/html" => {
            anyhow::bail!(
                "Радиосервер вернул HTML-страницу вместо аудио. Проверьте прямую ссылку на поток."
            );
        }
        _ => {}
    }

    let path = url.path().to_ascii_lowercase();
    if path.ends_with(".m3u")
        || path.ends_with(".m3u8")
        || path.ends_with(".pls")
        || path.ends_with(".asx")
    {
        anyhow::bail!(
            "URL оканчивается на файл плейлиста. Укажите прямой HTTP(S)-адрес аудиопотока."
        );
    }

    Ok(if path.ends_with(".aac") || path.ends_with(".aacp") {
        "audio/aac"
    } else if path.ends_with(".ogg") || path.ends_with(".oga") || path.ends_with(".opus") {
        "audio/ogg"
    } else if path.ends_with(".wav") {
        "audio/vnd.wav"
    } else if path.ends_with(".flac") {
        "audio/flac"
    } else {
        "audio/mpeg"
    })
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
            HttpStream::new(client, parsed.clone()),
        )
        .await
        .context("Тайм-аут: сервер радиостанции не ответил за 15 секунд")?
        .context("Не удалось установить HTTP-соединение с радиостанцией")?;

        let response_content_type = stream.content_type().map(str::to_owned);
        let mime_type = mime_hint(&parsed, response_content_type.as_deref())
            .context("Не удалось определить формат ответа радиосервера")?;

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
                    .with_mime_type(mime_type)
                    .build()
            }),
        )
        .await
        .context("Тайм-аут: радиосервер не передал заголовок аудиоданных за 10 секунд")?
        .context("Потоковый декодер завершился с ошибкой")?
        .context("Не удалось декодировать аудиопоток. Проверьте прямую ссылку и формат MP3/AAC/OGG")?;

        let output = DeviceSinkBuilder::open_default_sink()
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
    fn recognizes_common_radio_stream_formats_from_url() {
        assert_eq!(
            mime_hint(&url::Url::parse("https://radio.example/live.aac").unwrap(), None).unwrap(),
            "audio/aac"
        );
        assert_eq!(
            mime_hint(&url::Url::parse("https://radio.example/live.ogg").unwrap(), None).unwrap(),
            "audio/ogg"
        );
        assert_eq!(
            mime_hint(&url::Url::parse("https://radio.example/live.wav").unwrap(), None).unwrap(),
            "audio/vnd.wav"
        );
        assert_eq!(
            mime_hint(&url::Url::parse("https://europaplus.hostingradio.ru:8030/ep128").unwrap(), None).unwrap(),
            "audio/mpeg"
        );
    }

    #[test]
    fn prefers_the_actual_server_content_type() {
        let url = url::Url::parse("https://radio.example/live").unwrap();
        assert_eq!(
            mime_hint(&url, Some("audio/aacp; bitrate=128")).unwrap(),
            "audio/aac"
        );
        assert_eq!(
            mime_hint(&url, Some("audio/ogg; codecs=vorbis")).unwrap(),
            "audio/ogg"
        );
    }

    #[test]
    fn rejects_playlist_and_html_responses() {
        let url = url::Url::parse("https://radio.example/playlist").unwrap();
        assert!(mime_hint(&url, Some("audio/x-scpls")).is_err());
        assert!(mime_hint(&url, Some("text/html; charset=utf-8")).is_err());

        let playlist = url::Url::parse("https://radio.example/live.m3u8").unwrap();
        assert!(mime_hint(&playlist, None).is_err());
    }
}
