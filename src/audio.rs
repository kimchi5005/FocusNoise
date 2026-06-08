use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SampleFormat, Stream,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    model::{NoiseKind, PlaybackState},
    settings::Settings,
};

#[derive(Debug)]
struct AudioState {
    playback: PlaybackState,
    noise_kind: NoiseKind,
    target_volume: f32,
    current_volume: f32,
    balance: f32,
    fade_step: f32,
    brown_last: f32,
}

impl AudioState {
    fn from_settings(settings: &Settings) -> Self {
        Self {
            playback: PlaybackState::Stopped,
            noise_kind: settings.noise_kind,
            target_volume: settings.volume,
            current_volume: 0.0,
            balance: settings.balance,
            fade_step: 1.0 / (2.0 * 48_000.0),
            brown_last: 0.0,
        }
    }
}

pub struct AudioEngine {
    state: Arc<Mutex<AudioState>>,
    _stream: Stream,
    sample_rate: f32,
}

impl AudioEngine {
    pub fn new(settings: &Settings) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("No output audio device was found."))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let state = Arc::new(Mutex::new(AudioState::from_settings(settings)));
        if let Ok(mut audio) = state.lock() {
            audio.fade_step = fade_step(settings.fade_seconds, sample_rate);
        }

        let err_fn = |err| eprintln!("Audio stream error: {err}");
        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                build_stream::<f32>(&device, &config.into(), channels, state.clone(), err_fn)?
            }
            SampleFormat::I16 => {
                build_stream::<i16>(&device, &config.into(), channels, state.clone(), err_fn)?
            }
            SampleFormat::U16 => {
                build_stream::<u16>(&device, &config.into(), channels, state.clone(), err_fn)?
            }
            other => return Err(anyhow!("Unsupported sample format: {other:?}")),
        };
        stream.play()?;

        Ok(Self {
            state,
            _stream: stream,
            sample_rate,
        })
    }

    pub fn set_noise_kind(&self, kind: NoiseKind) {
        self.with_state(|state| state.noise_kind = kind);
    }

    pub fn set_volume(&self, volume: f32) {
        self.with_state(|state| {
            state.target_volume = volume;
            if matches!(state.playback, PlaybackState::Playing) {
                state.current_volume = volume;
            }
        });
    }

    pub fn set_balance(&self, balance: f32) {
        self.with_state(|state| state.balance = balance);
    }

    pub fn set_fade_seconds(&self, seconds: f32) {
        self.with_state(|state| state.fade_step = fade_step(seconds, self.sample_rate));
    }

    pub fn play(&self) {
        self.with_state(|state| {
            state.current_volume = state.target_volume;
            state.playback = PlaybackState::Playing;
        });
    }

    pub fn pause(&self) {
        self.with_state(|state| {
            state.current_volume = 0.0;
            state.playback = PlaybackState::Paused;
        });
    }

    pub fn stop(&self) {
        self.with_state(|state| {
            state.current_volume = 0.0;
            state.brown_last = 0.0;
            state.playback = PlaybackState::Stopped;
        });
    }

    pub fn fade_in(&self) {
        self.with_state(|state| {
            state.current_volume = 0.0;
            state.playback = PlaybackState::FadingIn;
        });
    }

    pub fn fade_out(&self) {
        self.with_state(|state| {
            if !matches!(
                state.playback,
                PlaybackState::Stopped | PlaybackState::Paused
            ) {
                state.playback = PlaybackState::FadingOut;
            }
        });
    }

    pub fn playback(&self) -> PlaybackState {
        self.state
            .lock()
            .map(|state| state.playback)
            .unwrap_or(PlaybackState::Stopped)
    }

    fn with_state(&self, update: impl FnOnce(&mut AudioState)) {
        if let Ok(mut state) = self.state.lock() {
            update(&mut state);
        }
    }
}

fn fade_step(seconds: f32, sample_rate: f32) -> f32 {
    if seconds <= 0.01 {
        return 1.0;
    }
    1.0 / (seconds * sample_rate)
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    channels: usize,
    state: Arc<Mutex<AudioState>>,
    err_fn: impl Fn(cpal::StreamError) + Send + 'static,
) -> Result<Stream>
where
    T: Sample + cpal::SizedSample,
    T: FromSample<f32>,
{
    let mut rng = SmallRng::from_entropy();

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            fill_audio(data, channels, &state, &mut rng);
        },
        err_fn,
        Some(Duration::from_millis(100)),
    )?;

    Ok(stream)
}

fn fill_audio<T>(
    data: &mut [T],
    channels: usize,
    state: &Arc<Mutex<AudioState>>,
    rng: &mut SmallRng,
) where
    T: Sample,
    T: FromSample<f32>,
{
    let Ok(mut audio) = state.lock() else {
        for sample in data.iter_mut() {
            *sample = T::EQUILIBRIUM;
        }
        return;
    };

    for frame in data.chunks_mut(channels) {
        let mut value = match audio.playback {
            PlaybackState::Stopped | PlaybackState::Paused => 0.0,
            PlaybackState::Playing | PlaybackState::FadingIn | PlaybackState::FadingOut => {
                next_noise_sample(&mut audio, rng)
            }
        };

        match audio.playback {
            PlaybackState::FadingIn => {
                audio.current_volume =
                    (audio.current_volume + audio.fade_step).min(audio.target_volume);
                if audio.current_volume >= audio.target_volume {
                    audio.playback = PlaybackState::Playing;
                }
            }
            PlaybackState::FadingOut => {
                audio.current_volume = (audio.current_volume - audio.fade_step).max(0.0);
                if audio.current_volume <= 0.0 {
                    audio.playback = PlaybackState::Stopped;
                    audio.brown_last = 0.0;
                }
            }
            _ => {}
        }

        value *= audio.current_volume;
        let (left_gain, right_gain) = balance_gains(audio.balance);

        for (index, sample) in frame.iter_mut().enumerate() {
            let balanced = if index % 2 == 0 {
                value * left_gain
            } else {
                value * right_gain
            };
            *sample = T::from_sample(balanced.clamp(-1.0, 1.0));
        }
    }
}

fn next_noise_sample(audio: &mut AudioState, rng: &mut SmallRng) -> f32 {
    let white = rng.gen_range(-1.0..=1.0);

    match audio.noise_kind {
        NoiseKind::White => white * 0.35,
        NoiseKind::Brown => {
            audio.brown_last = (audio.brown_last + white * 0.02).clamp(-1.0, 1.0);
            audio.brown_last * 0.65
        }
    }
}

fn balance_gains(balance: f32) -> (f32, f32) {
    let balance = balance.clamp(-1.0, 1.0);
    if balance < 0.0 {
        (1.0, 1.0 + balance)
    } else {
        (1.0 - balance, 1.0)
    }
}
