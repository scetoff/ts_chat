use anyhow::{Context, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SampleFormat, SizedSample,
};
use std::sync::{Arc, Mutex};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub type WhisperModel = Arc<WhisperContext>;

const WHISPER_SAMPLE_RATE: u32 = 16_000;
const MAX_RECORDING_SECONDS: usize = 12;

/// A one-shot microphone recording. Audio remains in memory and is never uploaded.
pub struct MicrophoneRecorder {
    stream: cpal::Stream,
    samples: Arc<Mutex<Vec<f32>>>,
    sample_rate: u32,
    channels: u16,
}

impl MicrophoneRecorder {
    pub fn start() -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .context("Не найдено устройство ввода (микрофон)")?;
        let supported = device.default_input_config()
            .context("Не удалось получить настройки микрофона")?;
        let sample_format = supported.sample_format();
        let config: cpal::StreamConfig = supported.into();
        let sample_rate = config.sample_rate.0;
        let channels = config.channels;
        let max_samples = sample_rate as usize * channels as usize * MAX_RECORDING_SECONDS;
        let samples = Arc::new(Mutex::new(Vec::with_capacity(max_samples.min(1_000_000))));

        let stream = match sample_format {
            SampleFormat::F32 => Self::build_stream::<f32>(&device, config, samples.clone(), max_samples)?,
            SampleFormat::I16 => Self::build_stream::<i16>(&device, config, samples.clone(), max_samples)?,
            SampleFormat::U16 => Self::build_stream::<u16>(&device, config, samples.clone(), max_samples)?,
            other => anyhow::bail!("Формат микрофона {other:?} пока не поддерживается; выберите устройство с PCM float или 16-bit input"),
        };

        stream.play().context("Не удалось начать запись с микрофона")?;
        Ok(Self { stream, samples, sample_rate, channels })
    }

    fn build_stream<T>(
        device: &cpal::Device,
        config: cpal::StreamConfig,
        samples: Arc<Mutex<Vec<f32>>>,
        max_samples: usize,
    ) -> Result<cpal::Stream>
    where
        T: SizedSample + Copy + Send + 'static,
        f32: FromSample<T>,
    {
        let error_callback = |error| eprintln!("Ошибка микрофона: {error}");
        let stream = device.build_input_stream::<T, _, _>(
            config,
            move |data: &[T], _| {
                if let Ok(mut buffer) = samples.lock() {
                    let remaining = max_samples.saturating_sub(buffer.len());
                    buffer.extend(data.iter().take(remaining).map(|sample| sample.to_sample::<f32>()));
                }
            },
            error_callback,
            None,
        ).context("Не удалось открыть поток микрофона")?;
        Ok(stream)
    }

    /// Stops the input stream, mixes channels to mono and resamples to 16 kHz.
    pub fn finish(self) -> Result<Vec<f32>> {
        let Self { stream, samples, sample_rate, channels } = self;
        drop(stream);

        if sample_rate == 0 || channels == 0 {
            anyhow::bail!("Микрофон вернул некорректную частоту или число каналов");
        }

        let interleaved = samples.lock()
            .map_err(|_| anyhow::anyhow!("Буфер микрофона заблокирован"))?
            .clone();
        if interleaved.is_empty() {
            anyhow::bail!("Микрофон не записал аудиоданные");
        }

        let channel_count = channels.max(1) as usize;
        let mono: Vec<f32> = interleaved
            .chunks_exact(channel_count)
            .map(|frame| frame.iter().copied().sum::<f32>() / channel_count as f32)
            .collect();

        if mono.is_empty() {
            anyhow::bail!("Микрофон не записал полных аудиокадров");
        }

        if sample_rate == WHISPER_SAMPLE_RATE {
            return Ok(mono);
        }

        let output_len = ((mono.len() as u64 * WHISPER_SAMPLE_RATE as u64) / sample_rate.max(1) as u64) as usize;
        if output_len == 0 {
            anyhow::bail!("Запись слишком короткая");
        }

        let scale = sample_rate as f64 / WHISPER_SAMPLE_RATE as f64;
        let mut output = Vec::with_capacity(output_len);
        for index in 0..output_len {
            let position = index as f64 * scale;
            let left = position.floor() as usize;
            let right = (left + 1).min(mono.len() - 1);
            let fraction = (position - left as f64) as f32;
            output.push(mono[left] * (1.0 - fraction) + mono[right] * fraction);
        }
        Ok(output)
    }
}

pub fn load_model(path: &str) -> Result<WhisperModel> {
    let context = WhisperContext::new_with_params(
        path,
        WhisperContextParameters::default(),
    ).context("Не удалось открыть Whisper-модель")?;
    if !context.is_multilingual() {
        anyhow::bail!("Нужна мультиязычная модель Whisper (например ggml-base.bin), а не вариант .en");
    }
    Ok(Arc::new(context))
}

pub fn transcribe(model: WhisperModel, audio: &[f32]) -> Result<String> {
    if audio.len() < WHISPER_SAMPLE_RATE as usize / 4 {
        anyhow::bail!("Запись слишком короткая — скажите команду ещё раз");
    }

    let mut state = model.create_state().context("Не удалось создать состояние Whisper")?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("ru"));
    params.set_translate(false);
    params.set_n_threads(4);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_no_context(true);

    state.full(params, audio).context("Ошибка распознавания речи")?;
    let text = state.as_iter()
        .map(|segment| segment.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let text = text.trim().to_string();
    if text.is_empty() {
        anyhow::bail!("Whisper не распознал речь");
    }
    Ok(text)
}
