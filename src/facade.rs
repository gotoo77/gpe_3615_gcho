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

const FUNCTION_KEYS: &[FunctionKeySpec] = &[
    FunctionKeySpec {
        key: FunctionKey::Summary,
        label: "SOMMAIRE",
        x: 7,
        y: 212,
        width: 61,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Return,
        label: "RETOUR",
        x: 71,
        y: 212,
        width: 48,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Correction,
        label: "CORRECT.",
        x: 122,
        y: 212,
        width: 73,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Guide,
        label: "GUIDE",
        x: 198,
        y: 212,
        width: 48,
        height: 21,
    },
    FunctionKeySpec {
        key: FunctionKey::Send,
        label: "ENVOI",
        x: 249,
        y: 212,
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
            x >= spec.x
                && x < spec.x + spec.width
                && y >= spec.y
                && y < spec.y + spec.height
        })
        .map(|spec| spec.key)
}
