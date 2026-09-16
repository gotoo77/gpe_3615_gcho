use gotoo_pixel_engine::{Audio, AudioError, SoundId};
use gpe_3615_gcho::{RetroCue, data_chunks_crossed, play_retro_cue, register_retro_audio};

#[test]
fn chunk_crossing_counts_exact_boundaries_without_frame_spam() {
    assert_eq!(data_chunks_crossed(0, 0, 16), 0);
    assert_eq!(data_chunks_crossed(0, 15, 16), 0);
    assert_eq!(data_chunks_crossed(15, 16, 16), 1);
    assert_eq!(data_chunks_crossed(16, 31, 16), 0);
    assert_eq!(data_chunks_crossed(16, 32, 16), 1);
    assert_eq!(data_chunks_crossed(0, 48, 16), 3);
    assert_eq!(data_chunks_crossed(48, 16, 16), 0);
}

#[derive(Default)]
struct RecordingAudio {
    registered: Vec<&'static str>,
    played: Vec<&'static str>,
}

impl Audio for RecordingAudio {
    fn register_wav(&mut self, id: SoundId, bytes: &[u8]) -> Result<(), AudioError> {
        assert!(bytes.len() > 44);
        assert_eq!(&bytes[..4], b"RIFF");
        self.registered.push(id.as_str());
        Ok(())
    }

    fn play(&mut self, id: SoundId) -> Result<(), AudioError> {
        self.played.push(id.as_str());
        Ok(())
    }
}

#[test]
fn data_chunk_is_a_registered_distinct_retro_cue() {
    let mut audio = RecordingAudio::default();
    register_retro_audio(&mut audio).expect("retro audio should register");

    assert_eq!(audio.registered.len(), 6);
    play_retro_cue(&mut audio, RetroCue::DataChunk).expect("data chunk cue should play");
    assert_eq!(audio.played.len(), 1);
    assert_eq!(audio.played[0], "gcho.data-chunk");
}
