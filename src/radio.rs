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
            HttpStream::new(client, parsed),
        )
        .await
        .context("Тайм-аут: сервер радиостанции не ответил за 15 секунд")?
        .context("Не удалось установить HTTP-соединение с радиостанцией")?;

        let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
        let mime_type = stream
            .content_type()
            .unwrap_or("audio/mpeg")
            .to_string();
        // Two seconds of initial audio is enough to start decoding without
        // making the first connection wait for a large live-stream buffer.
        let bitrate_kbps = icy_headers.bitrate().unwrap_or(128) as u64;
        let prefetch_bytes = (bitrate_kbps.saturating_mul(1024).saturating_mul(2) / 8)
            .clamp(16 * 1024, 128 * 1024);

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
