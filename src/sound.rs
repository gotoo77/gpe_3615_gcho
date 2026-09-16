use std::f32::consts::TAU;

use gotoo_pixel_engine::{Audio, AudioBus, AudioError, SoundId, pcm16_mono_wav};

const SAMPLE_RATE: u32 = 44_100;
const BOOT_SOUND: SoundId = SoundId::new("gcho.modem-connect");
const KEY_SOUND: SoundId = SoundId::new("gcho.key");
const SEND_SOUND: SoundId = SoundId::new("gcho.send");
const ERROR_SOUND: SoundId = SoundId::new("gcho.error");
const NAVIGATE_SOUND: SoundId = SoundId::new("gcho.navigate");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetroCue {
    Boot,
    Key,
    Send,
    Error,
    Navigate,
}

impl RetroCue {
    const fn sound_id(self) -> SoundId {
        match self {
            Self::Boot => BOOT_SOUND,
            Self::Key => KEY_SOUND,
            Self::Send => SEND_SOUND,
            Self::Error => ERROR_SOUND,
            Self::Navigate => NAVIGATE_SOUND,
        }
    }

    const fn bus(self) -> AudioBus {
        match self {
            Self::Boot => AudioBus::Sfx,
            Self::Key | Self::Send | Self::Error | Self::Navigate => AudioBus::Ui,
        }
    }
}

pub fn register_retro_audio(audio: &mut dyn Audio) -> Result<(), AudioError> {
    for (id, samples) in [
        (BOOT_SOUND, modem_connect_samples()),
        (KEY_SOUND, key_samples()),
        (SEND_SOUND, send_samples()),
        (ERROR_SOUND, error_samples()),
        (NAVIGATE_SOUND, navigate_samples()),
    ] {
        let wav = pcm16_mono_wav(SAMPLE_RATE, &samples)?;
        audio.register_wav(id, &wav)?;
    }
    Ok(())
}

pub fn play_retro_cue(audio: &mut dyn Audio, cue: RetroCue) -> Result<(), AudioError> {
    audio.play_on_bus(cue.sound_id(), cue.bus())
}

fn modem_connect_samples() -> Vec<i16> {
    let mut samples = Vec::new();
    append_tone(&mut samples, 260, &[(350.0, 0.48), (440.0, 0.42)], 0.20);
    append_silence(&mut samples, 90);

    for (low, high) in [
        (697.0, 1209.0),
        (697.0, 1336.0),
        (770.0, 1209.0),
        (852.0, 1477.0),
    ] {
        append_tone(&mut samples, 72, &[(low, 0.5), (high, 0.5)], 0.24);
        append_silence(&mut samples, 36);
    }

    append_sweep(&mut samples, 250, 480.0, 2_200.0, 0.22);
    append_noise_carrier(&mut samples, 170, 1_750.0, 0.18, 0x3615_6c68);
    append_tone(
        &mut samples,
        220,
        &[(1_800.0, 0.55), (2_400.0, 0.35), (600.0, 0.10)],
        0.18,
    );
    append_noise_carrier(&mut samples, 260, 2_100.0, 0.15, 0x28_800);
    append_sweep(&mut samples, 210, 2_600.0, 900.0, 0.18);
    append_tone(&mut samples, 180, &[(1_650.0, 0.6), (2_050.0, 0.4)], 0.15);
    append_silence(&mut samples, 80);
    append_tone(&mut samples, 110, &[(1_200.0, 1.0)], 0.12);
    samples
}

fn key_samples() -> Vec<i16> {
    let mut samples = Vec::new();
    append_tone(&mut samples, 42, &[(920.0, 1.0)], 0.16);
    append_tone(&mut samples, 18, &[(680.0, 1.0)], 0.08);
    samples
}

fn send_samples() -> Vec<i16> {
    let mut samples = Vec::new();
    append_tone(&mut samples, 54, &[(760.0, 1.0)], 0.14);
    append_tone(&mut samples, 72, &[(1_180.0, 1.0)], 0.18);
    samples
}

fn error_samples() -> Vec<i16> {
    let mut samples = Vec::new();
    append_tone(&mut samples, 72, &[(310.0, 0.7), (620.0, 0.3)], 0.18);
    append_silence(&mut samples, 28);
    append_tone(&mut samples, 92, &[(235.0, 0.75), (470.0, 0.25)], 0.16);
    samples
}

fn navigate_samples() -> Vec<i16> {
    let mut samples = Vec::new();
    append_sweep(&mut samples, 82, 620.0, 980.0, 0.13);
    samples
}

fn append_tone(samples: &mut Vec<i16>, duration_ms: u32, tones: &[(f32, f32)], gain: f32) {
    let count = samples_for_ms(duration_ms);
    for index in 0..count {
        let t = index as f32 / SAMPLE_RATE as f32;
        let envelope = short_envelope(index, count);
        let value = tones
            .iter()
            .map(|(frequency, weight)| (TAU * frequency * t).sin() * weight)
            .sum::<f32>();
        samples.push(to_sample(value * gain * envelope));
    }
}

fn append_sweep(samples: &mut Vec<i16>, duration_ms: u32, from: f32, to: f32, gain: f32) {
    let count = samples_for_ms(duration_ms);
    let mut phase = 0.0_f32;
    for index in 0..count {
        let progress = index as f32 / count.max(1) as f32;
        let frequency = from + (to - from) * progress;
        phase += TAU * frequency / SAMPLE_RATE as f32;
        samples.push(to_sample(phase.sin() * gain * short_envelope(index, count)));
    }
}

fn append_noise_carrier(
    samples: &mut Vec<i16>,
    duration_ms: u32,
    carrier_hz: f32,
    gain: f32,
    mut state: u32,
) {
    let count = samples_for_ms(duration_ms);
    for index in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let noise = ((state >> 8) as f32 / 16_777_215.0) * 2.0 - 1.0;
        let t = index as f32 / SAMPLE_RATE as f32;
        let carrier = (TAU * carrier_hz * t).sin();
        let value = carrier * 0.68 + noise * 0.32;
        samples.push(to_sample(value * gain * short_envelope(index, count)));
    }
}

fn append_silence(samples: &mut Vec<i16>, duration_ms: u32) {
    samples.resize(samples.len() + samples_for_ms(duration_ms), 0);
}

fn samples_for_ms(duration_ms: u32) -> usize {
    (SAMPLE_RATE as usize * duration_ms as usize) / 1_000
}

fn short_envelope(index: usize, count: usize) -> f32 {
    let edge = (SAMPLE_RATE as usize / 250).min(count / 2).max(1);
    if index < edge {
        index as f32 / edge as f32
    } else if index + edge > count {
        (count - index) as f32 / edge as f32
    } else {
        1.0
    }
}

fn to_sample(value: f32) -> i16 {
    (value.clamp(-1.0, 1.0) * i16::MAX as f32) as i16
}
