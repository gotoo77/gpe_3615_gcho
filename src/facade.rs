use crate::service::NavCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionKey {
    Summary,
    Return,
    Correction,
    Guide,
    Send,
}

impl FunctionKey {
    pub const fn command(self) -> NavCommand {
        match self {
            Self::Summary => NavCommand::Summary,
            Self::Return => NavCommand::Return,
            Self::Correction => NavCommand::Correction,
            Self::Guide => NavCommand::Guide,
            Self::Send => NavCommand::Send,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionKeySpec {
    pub key: FunctionKey,
    pub label: &'static str,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub const FUNCTION_KEYS_TOP_Y: i32 = 212;

const FUNCTION_KEYS: &[FunctionKeySpec] = &[
    FunctionKeySpec {
        key: FunctionKey::Summary,
        label: "SOMMAIRE",
        x: 7,
        y: FUNCTION_KEYS_TOP_Y,
        width: 61,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Return,
        label: "RETOUR",
        x: 71,
        y: FUNCTION_KEYS_TOP_Y,
        width: 48,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Correction,
        label: "CORRECT.",
        x: 122,
        y: FUNCTION_KEYS_TOP_Y,
        width: 73,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Guide,
        label: "GUIDE",
        x: 198,
        y: FUNCTION_KEYS_TOP_Y,
        width: 48,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Send,
        label: "ENVOI",
        x: 249,
        y: FUNCTION_KEYS_TOP_Y,
        width: 64,
        height: 21,
    },
];

pub const fn function_keys() -> &'static [FunctionKeySpec] {
    FUNCTION_KEYS
}

pub fn function_key_at(x: i32, y: i32) -> Option<FunctionKey> {
    FUNCTION_KEYS
        .iter()
        .find(|spec| {
            x >= spec.x && x < spec.x + spec.width && y >= spec.y && y < spec.y + spec.height
        })
        .map(|spec| spec.key)
}

pub fn snapshot_date_label(generated_at: &str) -> String {
    let date = generated_at.get(..10);
    let Some(date) = date else {
        return "SNAPSHOT : LOCAL".into();
    };
    let bytes = date.as_bytes();
    let valid = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit());
    if !valid {
        return "SNAPSHOT : LOCAL".into();
    }

    format!("SNAPSHOT : {}/{}/{}", &date[8..10], &date[5..7], &date[..4])
}
