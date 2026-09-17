use crate::branding::{
    ACCUEIL_MENU_LEFT_X, ACCUEIL_MENU_RIGHT_X, ACCUEIL_MENU_ROW_HEIGHT, ACCUEIL_MENU_START_Y,
    ACCUEIL_MENU_STEP,
};
use crate::service::{NavCommand, PageId, Service};

const MENU_LEFT_X: i32 = 9;
const MENU_RIGHT_X: i32 = 311;

pub fn menu_entry_at(service: &Service, x: i32, y: i32) -> Option<u8> {
    if service.current_detail().is_some() {
        return None;
    }

    let (left, right, start_y, step, top_padding, height) = match service.current_page() {
        PageId::Accueil => (
            ACCUEIL_MENU_LEFT_X,
            ACCUEIL_MENU_RIGHT_X,
            ACCUEIL_MENU_START_Y,
            ACCUEIL_MENU_STEP,
            2,
            ACCUEIL_MENU_ROW_HEIGHT,
        ),
        _ => (MENU_LEFT_X, MENU_RIGHT_X, 52, 15, 3, 12),
    };

    if !(left..right).contains(&x) {
        return None;
    }

    service
        .entries()
        .iter()
        .enumerate()
        .find_map(|(index, entry)| {
            let row_y = start_y + index as i32 * step;
            let top = row_y - top_padding;
            (y >= top && y < top + height).then_some(entry.key)
        })
}

pub fn menu_tap_command(service: &Service, x: i32, y: i32) -> Option<NavCommand> {
    let key = menu_entry_at(service, x, y)?;
    if service.pending_digit() == Some(key) {
        Some(NavCommand::Send)
    } else {
        Some(NavCommand::Digit(key))
    }
}
