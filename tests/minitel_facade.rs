use gotoo_pixel_engine::{Audio, AudioError, SoundId};
use gpe_3615_gcho::{
    FunctionKey, NavCommand, PageId, RetroCue, Service, function_key_at, function_keys,
    play_retro_cue, register_retro_audio, snapshot_date_label,
};

#[test]
fn minitel_function_keys_are_visible_and_hit_testable() {
    let keys = function_keys();
    assert_eq!(keys.len(), 5);
    assert_eq!(keys[0].key, FunctionKey::Summary);
    assert_eq!(keys[1].key, FunctionKey::Return);
    assert_eq!(keys[2].key, FunctionKey::Correction);
    assert_eq!(keys[3].key, FunctionKey::Guide);
    assert_eq!(keys[4].key, FunctionKey::Send);

    for spec in keys {
        let x = spec.x + spec.width / 2;
        let y = spec.y + spec.height / 2;
        assert_eq!(function_key_at(x, y), Some(spec.key));
    }

    assert_eq!(function_key_at(0, 0), None);
}

#[test]
fn guide_is_a_real_navigation_command() {
    let mut service = Service::new();
    service.apply(NavCommand::Guide);
    assert_eq!(service.current_page(), PageId::Aide);
}

#[test]
fn each_function_key_maps_to_the_expected_command() {
    assert_eq!(FunctionKey::Summary.command(), NavCommand::Summary);
    assert_eq!(FunctionKey::Return.command(), NavCommand::Return);
    assert_eq!(FunctionKey::Correction.command(), NavCommand::Correction);
    assert_eq!(FunctionKey::Guide.command(), NavCommand::Guide);
    assert_eq!(FunctionKey::Send.command(), NavCommand::Send);
}

#[derive(Default)]
struct RecordingAudio {
    registered: Vec<(&'static str, usize, [u8; 4])>,
    played: Vec<&'static str>,
}

impl Audio for RecordingAudio {
    fn register_wav(&mut self, id: SoundId, bytes: &[u8]) -> Result<(), AudioError> {
        let mut prefix = [0_u8; 4];
        prefix.copy_from_slice(&bytes[..4]);
        self.registered.push((id.as_str(), bytes.len(), prefix));
        Ok(())
    }

    fn play(&mut self, id: SoundId) -> Result<(), AudioError> {
        self.played.push(id.as_str());
        Ok(())
    }
}

#[test]
fn retro_audio_catalog_is_local_wav_and_has_distinct_cues() {
    let mut audio = RecordingAudio::default();
    register_retro_audio(&mut audio).expect("retro audio should register");

    assert_eq!(audio.registered.len(), 5);
    assert!(audio.registered.iter().all(|(_, len, prefix)| {
        *len > 44 && prefix == b"RIFF"
    }));

    for cue in [
        RetroCue::Boot,
        RetroCue::Key,
        RetroCue::Send,
        RetroCue::Error,
        RetroCue::Navigate,
    ] {
        play_retro_cue(&mut audio, cue).expect("retro cue should play");
    }

    assert_eq!(audio.played.len(), 5);
    let mut distinct = audio.played.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(distinct.len(), 5);
}

#[test]
fn editorial_timestamp_is_presented_as_a_snapshot_date() {
    assert_eq!(
        snapshot_date_label("2026-09-15T00:00:00Z"),
        "SNAPSHOT : 15/09/2026"
    );
    assert_eq!(snapshot_date_label("bundled-fallback"), "SNAPSHOT : LOCAL");
}
