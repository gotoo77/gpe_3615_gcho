use gotoo_pixel_engine::{Frame, Game, GameResult, Key, TextInputEvent};

use crate::content::ContentBundle;
use crate::render::{render_boot, render_terminal};
use crate::service::{NavCommand, Service};

const BOOT_DURATION_SECONDS: f32 = 1.8;

pub struct GchoApp {
    service: Service,
    content: ContentBundle,
    boot_elapsed: f32,
    blink_elapsed: f32,
}

impl Default for GchoApp {
    fn default() -> Self {
        Self::new()
    }
}

impl GchoApp {
    pub fn new() -> Self {
        Self {
            service: Service::new(),
            content: ContentBundle::load_bundled(),
            boot_elapsed: 0.0,
            blink_elapsed: 0.0,
        }
    }
}

impl Game for GchoApp {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        let dt = frame.delta_time.as_secs_f32();
        self.blink_elapsed = (self.blink_elapsed + dt) % 1.0;

        if self.boot_elapsed < BOOT_DURATION_SECONDS {
            self.boot_elapsed += dt;
            if frame.input.key(Key::Enter).pressed() || frame.input.key(Key::Escape).pressed() {
                self.boot_elapsed = BOOT_DURATION_SECONDS;
            }
            render_boot(
                frame.framebuffer,
                (self.boot_elapsed / BOOT_DURATION_SECONDS).clamp(0.0, 1.0),
            );
            return GameResult::Continue;
        }

        for event in frame.input.text_events() {
            match event {
                TextInputEvent::Insert(text) => {
                    for digit in text.chars().filter_map(|character| character.to_digit(10)) {
                        self.service.apply(NavCommand::Digit(digit as u8));
                    }
                }
                TextInputEvent::Backspace | TextInputEvent::Delete => {
                    self.service.apply(NavCommand::Correction)
                }
                TextInputEvent::Home => self.service.apply(NavCommand::Summary),
                TextInputEvent::Left | TextInputEvent::Right | TextInputEvent::End => {}
            }
        }

        if frame.input.key(Key::Enter).pressed() {
            self.service.apply(NavCommand::Send);
        }
        if frame.input.key(Key::Escape).pressed() {
            self.service.apply(NavCommand::Return);
        }

        if self.service.take_exit_requested() {
            return GameResult::Exit;
        }

        render_terminal(
            frame.framebuffer,
            &self.service,
            &self.content,
            self.blink_elapsed < 0.55,
        );
        GameResult::Continue
    }
}
