use gotoo_pixel_engine::{Framebuffer, Pixel};

const TERMINAL_BG: Pixel = Pixel::rgb(2, 7, 10);
const TERMINAL_DIM: Pixel = Pixel::rgb(20, 48, 55);
const TERMINAL_OFF_WHITE: Pixel = Pixel::rgb(225, 232, 218);
const TERMINAL_YELLOW: Pixel = Pixel::rgb(245, 215, 70);
const MINITEL_COLUMNS: usize = 40;
const CELL_HEIGHT: usize = 10;

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

pub const fn public_service_slogan() -> &'static str {
    "DITES-NOUS DE QUOI VOUS AVEZ BESOIN. NOUS VOUS DIRONS COMMENT VOUS EN PASSER."
}

pub(crate) fn render_public_service_banner(framebuffer: &mut Framebuffer) {
    framebuffer.fill_rect(9, 166, 302, 43, Pixel::rgb(8, 24, 28));
    framebuffer.draw_rect(9, 166, 302, 43, TERMINAL_YELLOW);
    framebuffer.draw_text(14, 171, "MESSAGE DE SERVICE PUBLIC", TERMINAL_YELLOW);
    framebuffer.draw_text(
        14,
        184,
        "DITES-NOUS DE QUOI VOUS AVEZ BESOIN.",
        TERMINAL_OFF_WHITE,
    );
    framebuffer.draw_text(
        14,
        196,
        "NOUS VOUS DIRONS COMMENT VOUS EN PASSER.",
        TERMINAL_DIM,
    );
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
