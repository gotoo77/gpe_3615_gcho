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
        (baud > 0).then_some(Self {
            baud,
            response_delay_ms,
        })
    }

    pub fn from_baud(baud: u32, response_delay_ms: u32) -> Self {
        Self::try_from_baud(baud, response_delay_ms).expect("baud must be greater than zero")
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
