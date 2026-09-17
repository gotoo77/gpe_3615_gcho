mod app;
mod branding;
mod content;
mod experience;
mod facade;
mod render;
mod service;
mod sound;
mod touch;

pub use app::{
    BOOT_DURATION_SECONDS, BRANDING_SPLASH_DURATION_SECONDS, GchoApp, StartupStage, startup_stage,
};
pub use branding::{ACCUEIL_LOGO_RECT, decode_macronus_portrait, decode_minitel_logo};
pub use content::{ContentBundle, ContentFile, EditorialMessage};
pub use experience::{
    DATA_CHUNK_CHARACTERS, LIVE_STATUS_HEIGHT, LIVE_STATUS_TOP_Y, LiveServicePulse, TerminalTiming,
    data_chunks_crossed, public_service_slogan, selection_hint,
};
pub use facade::{
    FUNCTION_KEYS_TOP_Y, FunctionKey, FunctionKeySpec, function_key_at, function_keys,
    snapshot_date_label,
};
pub use render::{NODE7_FILE_FINAL_Y, NOTICE_HEIGHT, NOTICE_TOP_Y, choice_prompt_y};
pub use service::{DetailId, Entry, NavCommand, PageId, Service};
pub use sound::{RetroCue, play_retro_cue, register_retro_audio};
pub use touch::{menu_entry_at, menu_tap_command};

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
