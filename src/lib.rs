mod app;
mod branding;
mod content;
mod experience;
mod facade;
mod render;
mod service;
mod sound;

pub use app::GchoApp;
pub use branding::{ACCUEIL_LOGO_RECT, decode_minitel_logo};
pub use content::{ContentBundle, ContentFile, EditorialMessage};
pub use experience::{
    DATA_CHUNK_CHARACTERS, LiveServicePulse, TerminalTiming, data_chunks_crossed,
    public_service_slogan, selection_hint,
};
pub use facade::{
    FunctionKey, FunctionKeySpec, function_key_at, function_keys, snapshot_date_label,
};
pub use service::{DetailId, Entry, NavCommand, PageId, Service};
pub use sound::{RetroCue, play_retro_cue, register_retro_audio};

use gotoo_pixel_engine::EngineConfig;

pub const FRAMEBUFFER_WIDTH: u32 = 320;
pub const FRAMEBUFFER_HEIGHT: u32 = 240;

pub fn engine_config() -> EngineConfig {
    EngineConfig {
        title: "3615 GCHO".into(),
        framebuffer_width: FRAMEBUFFER_WIDTH,
        framebuffer_height: FRAMEBUFFER_HEIGHT,
        window_width: 960,
        window_height: 720,
    }
}
