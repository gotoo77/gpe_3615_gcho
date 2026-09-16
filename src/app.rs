use gotoo_pixel_engine::{Audio, Frame, Game, GameResult, Key, MouseButton, TextInputEvent};

use crate::content::ContentBundle;
use crate::experience::{
    LiveServicePulse, TerminalTiming, render_live_status, render_public_service_banner,
    render_transmission_mask,
};
use crate::facade::{FunctionKey, function_key_at};
use crate::render::{render_boot, render_terminal};
use crate::service::{NavCommand, PageId, Service};
use crate::sound::{RetroCue, play_retro_cue, register_retro_audio};

const BOOT_DURATION_SECONDS: f32 = 2.6;
const FUNCTION_KEY_FLASH_SECONDS: f32 = 0.13;

pub struct GchoApp {
    service: Service,
    content: ContentBundle,
    timing: TerminalTiming,
    live_pulse: LiveServicePulse,
    boot_elapsed: f32,
    blink_elapsed: f32,
    transmission_elapsed: f32,
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
        Self::with_timing(TerminalTiming::default())
    }

    pub fn with_timing(timing: TerminalTiming) -> Self {
        Self {
            service: Service::new(),
            content: ContentBundle::load_bundled(),
            timing,
            live_pulse: LiveServicePulse::new(),
            boot_elapsed: 0.0,
            blink_elapsed: 0.0,
            transmission_elapsed: 0.0,
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
        let previous_view = (self.service.current_page(), self.service.current_detail());
        self.service.apply(command);
        let current_view = (self.service.current_page(), self.service.current_detail());
        if current_view != previous_view {
            self.transmission_elapsed = 0.0;
        }

        let cue = if self.service.notice().is_some() {
            RetroCue::Error
        } else {
            match command {
                NavCommand::Digit(_)
                | NavCommand::Next
                | NavCommand::Previous
                | NavCommand::Correction => RetroCue::Key,
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

        self.transmission_elapsed += dt;
        self.live_pulse.advance(dt);

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
                TextInputEvent::Left => {
                    commands.push((NavCommand::Return, Some(FunctionKey::Return)))
                }
                TextInputEvent::Right | TextInputEvent::End => {}
            }
        }

        if frame.input.key(Key::Up).pressed() {
            commands.push((NavCommand::Previous, None));
        }
        if frame.input.key(Key::Down).pressed() {
            commands.push((NavCommand::Next, None));
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
        if self.service.current_page() == PageId::Accueil
            && self.service.current_detail().is_none()
            && self.service.notice().is_none()
        {
            let message = self
                .live_pulse
                .banner_index(self.content.service_public.messages.len())
                .and_then(|index| self.content.service_public.messages.get(index));
            render_public_service_banner(frame.framebuffer, message);
        }
        render_transmission_mask(
            frame.framebuffer,
            self.timing.visible_characters(self.transmission_elapsed),
        );
        render_live_status(
            frame.framebuffer,
            self.live_pulse.status(),
            self.service.pending_digit(),
        );
        GameResult::Continue
    }
}
