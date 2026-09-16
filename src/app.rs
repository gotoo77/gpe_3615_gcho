use gotoo_pixel_engine::{Audio, Frame, Game, GameResult, Key, MouseButton, TextInputEvent};

use crate::content::ContentBundle;
use crate::facade::{FunctionKey, function_key_at};
use crate::render::{render_boot, render_terminal};
use crate::service::{NavCommand, Service};
use crate::sound::{RetroCue, play_retro_cue, register_retro_audio};

const BOOT_DURATION_SECONDS: f32 = 2.6;
const FUNCTION_KEY_FLASH_SECONDS: f32 = 0.13;

pub struct GchoApp {
    service: Service,
    content: ContentBundle,
    boot_elapsed: f32,
    blink_elapsed: f32,
    audio_registration_attempted: bool,
    boot_cue_attempted: bool,
    active_function_key: Option<FunctionKey>,
    function_key_flash: f32,
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
            audio_registration_attempted: false,
            boot_cue_attempted: false,
            active_function_key: None,
            function_key_flash: 0.0,
        }
    }

    fn apply_command(
        &mut self,
        audio: &mut dyn Audio,
        command: NavCommand,
        function_key: Option<FunctionKey>,
    ) {
        self.service.apply(command);
        let cue = if self.service.notice().is_some() {
            RetroCue::Error
        } else {
            match command {
                NavCommand::Digit(_) | NavCommand::Correction => RetroCue::Key,
                NavCommand::Send => RetroCue::Send,
                NavCommand::Return | NavCommand::Summary | NavCommand::Guide => RetroCue::Navigate,
            }
        };
        let _ = play_retro_cue(audio, cue);

        if let Some(function_key) = function_key {
            self.active_function_key = Some(function_key);
            self.function_key_flash = FUNCTION_KEY_FLASH_SECONDS;
        }
    }
}

impl Game for GchoApp {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        let dt = frame.delta_time.as_secs_f32();
        self.blink_elapsed = (self.blink_elapsed + dt) % 1.0;
        self.function_key_flash = (self.function_key_flash - dt).max(0.0);
        if self.function_key_flash == 0.0 {
            self.active_function_key = None;
        }

        if !self.audio_registration_attempted {
            self.audio_registration_attempted = true;
            let _ = register_retro_audio(frame.audio);
        }
        if !self.boot_cue_attempted {
            self.boot_cue_attempted = true;
            let _ = play_retro_cue(frame.audio, RetroCue::Boot);
        }

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

        let mut commands = Vec::new();
        for event in frame.input.text_events() {
            match event {
                TextInputEvent::Insert(text) => {
                    for digit in text.chars().filter_map(|character| character.to_digit(10)) {
                        commands.push((NavCommand::Digit(digit as u8), None));
                    }
                }
                TextInputEvent::Backspace | TextInputEvent::Delete => {
                    commands.push((NavCommand::Correction, Some(FunctionKey::Correction)))
                }
                TextInputEvent::Home => {
                    commands.push((NavCommand::Summary, Some(FunctionKey::Summary)))
                }
                TextInputEvent::Left | TextInputEvent::Right | TextInputEvent::End => {}
            }
        }

        if frame.input.key(Key::Enter).pressed() {
            commands.push((NavCommand::Send, Some(FunctionKey::Send)));
        }
        if frame.input.key(Key::Escape).pressed() {
            commands.push((NavCommand::Return, Some(FunctionKey::Return)));
        }
        if frame.input.key(Key::H).pressed() {
            commands.push((NavCommand::Guide, Some(FunctionKey::Guide)));
        }
        if frame.input.mouse_button(MouseButton::Left).pressed()
            && let Some((x, y)) = frame.input.mouse_position()
            && let Some(function_key) = function_key_at(x, y)
        {
            commands.push((function_key.command(), Some(function_key)));
        }

        for (command, function_key) in commands {
            self.apply_command(frame.audio, command, function_key);
        }

        if self.service.take_exit_requested() {
            return GameResult::Exit;
        }

        render_terminal(
            frame.framebuffer,
            &self.service,
            &self.content,
            self.blink_elapsed < 0.55,
            self.active_function_key,
        );
        GameResult::Continue
    }
}
