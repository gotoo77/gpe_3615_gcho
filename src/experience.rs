use gotoo_pixel_engine::{Framebuffer, Pixel};

use crate::content::EditorialMessage;

const TERMINAL_BG: Pixel = Pixel::rgb(2, 7, 10);
const TERMINAL_DIM: Pixel = Pixel::rgb(20, 48, 55);
const TERMINAL_OFF_WHITE: Pixel = Pixel::rgb(225, 232, 218);
const TERMINAL_CYAN: Pixel = Pixel::rgb(50, 220, 225);
const TERMINAL_YELLOW: Pixel = Pixel::rgb(245, 215, 70);
const MINITEL_COLUMNS: usize = 40;
const CELL_HEIGHT: usize = 10;
const BANNER_PERIOD_SECONDS: f32 = 8.0;
const STATUS_PERIOD_SECONDS: f32 = 12.0;
pub const DATA_CHUNK_CHARACTERS: usize = 16;
const LIVE_STATUSES: &[&str] = &[
    "RESEAU GCHO : OUVERT",
    "1 NOUVEAU MESSAGE",
    "NOEUD 7 : ACTIVITE DETECTEE",
    "ALERTE : FORMULAIRE NON RECU",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalTiming {
    baud: u32,
    response_delay_ms: u32,
}

impl Default for TerminalTiming {
    fn default() -> Self {
        Self::from_baud(1_200, 180)
    }
}

impl TerminalTiming {
    pub fn try_from_baud(baud: u32, response_delay_ms: u32) -> Option<Self> {
        (baud >= 10).then_some(Self {
            baud,
            response_delay_ms,
        })
    }

    pub fn from_baud(baud: u32, response_delay_ms: u32) -> Self {
        Self::try_from_baud(baud, response_delay_ms).expect("baud must be at least 10")
    }

    pub const fn baud(self) -> u32 {
        self.baud
    }

    pub const fn response_delay_ms(self) -> u32 {
        self.response_delay_ms
    }

    pub const fn characters_per_second(self) -> u32 {
        self.baud / 10
    }

    pub fn visible_characters(self, elapsed_seconds: f32) -> usize {
        let response_delay = self.response_delay_ms as f32 / 1_000.0;
        if elapsed_seconds <= response_delay {
            return 0;
        }
        let transmit_seconds = elapsed_seconds - response_delay;
        (transmit_seconds * self.characters_per_second() as f32).floor() as usize
    }
}

pub fn data_chunks_crossed(
    previous_visible: usize,
    current_visible: usize,
    chunk_characters: usize,
) -> usize {
    if chunk_characters == 0 || current_visible <= previous_visible {
        return 0;
    }

    (current_visible / chunk_characters).saturating_sub(previous_visible / chunk_characters)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiveServicePulse {
    elapsed_seconds: f32,
}

impl Default for LiveServicePulse {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveServicePulse {
    pub const fn new() -> Self {
        Self {
            elapsed_seconds: 0.0,
        }
    }

    pub fn advance(&mut self, delta_seconds: f32) {
        if delta_seconds.is_finite() && delta_seconds > 0.0 {
            self.elapsed_seconds += delta_seconds;
        }
    }

    pub fn banner_index(self, message_count: usize) -> Option<usize> {
        if message_count == 0 {
            return None;
        }
        let slot_count = message_count + 1;
        let slot = ((self.elapsed_seconds / BANNER_PERIOD_SECONDS).floor() as usize) % slot_count;
        slot.checked_sub(1)
    }

    pub fn status(self) -> &'static str {
        let slot =
            ((self.elapsed_seconds / STATUS_PERIOD_SECONDS).floor() as usize) % LIVE_STATUSES.len();
        LIVE_STATUSES[slot]
    }
}

pub const fn public_service_slogan() -> &'static str {
    "DITES-NOUS DE QUOI VOUS AVEZ BESOIN. NOUS VOUS DIRONS COMMENT VOUS EN PASSER."
}

pub fn selection_hint(selection: Option<u8>) -> String {
    match selection {
        Some(digit) => format!("SELECTION : {digit} / ENVOI"),
        None => "FLECHES HAUT/BAS : NAVIGUER".into(),
    }
}

pub(crate) fn render_public_service_banner(
    framebuffer: &mut Framebuffer,
    message: Option<&EditorialMessage>,
) {
    framebuffer.fill_rect(9, 160, 302, 38, Pixel::rgb(8, 24, 28));
    framebuffer.draw_rect(9, 160, 302, 38, TERMINAL_YELLOW);

    if let Some(message) = message {
        framebuffer.draw_text(14, 165, &fit_line(&message.title, 46), TERMINAL_YELLOW);
        if let Some(line) = message.body.first() {
            framebuffer.draw_text(14, 177, &fit_line(line, 46), TERMINAL_OFF_WHITE);
        }
        if let Some(line) = message.body.get(1) {
            framebuffer.draw_text(14, 188, &fit_line(line, 46), TERMINAL_DIM);
        }
    } else {
        framebuffer.draw_text(14, 165, "MESSAGE DE SERVICE PUBLIC", TERMINAL_YELLOW);
        framebuffer.draw_text(
            14,
            177,
            "DITES-NOUS DE QUOI VOUS AVEZ BESOIN.",
            TERMINAL_OFF_WHITE,
        );
        framebuffer.draw_text(
            14,
            188,
            "NOUS VOUS DIRONS COMMENT VOUS EN PASSER.",
            TERMINAL_DIM,
        );
    }
}

pub(crate) fn render_live_status(
    framebuffer: &mut Framebuffer,
    status: &str,
    selection: Option<u8>,
) {
    framebuffer.fill_rect(9, 200, 302, 10, Pixel::rgb(5, 16, 20));
    framebuffer.draw_line(9, 200, 310, 200, TERMINAL_DIM);
    let label = if selection.is_some() {
        selection_hint(selection)
    } else {
        status.to_owned()
    };
    framebuffer.draw_text(14, 202, &fit_line(&label, 46), TERMINAL_CYAN);
}

pub(crate) fn render_transmission_mask(framebuffer: &mut Framebuffer, visible_characters: usize) {
    let revealed_rows = visible_characters / MINITEL_COLUMNS;
    let revealed_height = revealed_rows.saturating_mul(CELL_HEIGHT);
    let height = framebuffer.height() as usize;
    if revealed_height >= height {
        return;
    }

    let y = revealed_height as i32;
    framebuffer.fill_rect(
        0,
        y,
        framebuffer.width(),
        (height - revealed_height) as u32,
        TERMINAL_BG,
    );
}

fn fit_line(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}
