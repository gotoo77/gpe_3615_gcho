mod app;
mod content;
mod render;
mod service;

pub use app::GchoApp;
pub use content::{ContentBundle, ContentFile, EditorialMessage};
pub use service::{DetailId, Entry, NavCommand, PageId, Service};

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
