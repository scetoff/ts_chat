use anyhow::{Context, Result};
use std::io::BufReader;
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};
use std::num::NonZeroUsize;
use stream_download::{storage::{adaptive::AdaptiveStorageProvider, temp::TempStorageProvider}, Settings, StreamDownload};

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

        let reader = StreamDownload::new_http(
            parsed,
            AdaptiveStorageProvider::new(
                TempStorageProvider::default(),
                NonZeroUsize::new(1024 * 1024).expect("non-zero radio buffer"),
            ),
            Settings::default(),
        )
        .await
        .map_err(|error| anyhow::anyhow!("Не удалось подключить радиопоток: {error}"))?;

        let decoder = Decoder::try_from(BufReader::new(reader))
            .context("Не удалось определить или декодировать аудиоформат радиостанции")?;

        let output = DeviceSinkBuilder::open_default_sink()
            .context("Не удалось открыть устройство вывода звука")?;
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
