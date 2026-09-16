use gpe_3615_gcho::{NavCommand, PageId, Service, menu_entry_at, menu_tap_command};

fn open_page(service: &mut Service, digit: u8) {
    service.apply(NavCommand::Digit(digit));
    service.apply(NavCommand::Send);
}

#[test]
fn accueil_rows_are_touchable_in_the_visible_royal_layout() {
    let service = Service::new();

    assert_eq!(menu_entry_at(&service, 120, 94), Some(1));
    assert_eq!(menu_entry_at(&service, 120, 110), Some(2));
    assert_eq!(menu_entry_at(&service, 120, 174), Some(6));
    assert_eq!(menu_entry_at(&service, 20, 94), None);
    assert_eq!(menu_entry_at(&service, 120, 84), None);
    assert_eq!(menu_entry_at(&service, 120, 188), None);
}

#[test]
fn normal_service_rows_are_touchable_at_their_rendered_positions() {
    let mut service = Service::new();
    open_page(&mut service, 6);
    assert_eq!(service.current_page(), PageId::Services);

    assert_eq!(menu_entry_at(&service, 20, 52), Some(1));
    assert_eq!(menu_entry_at(&service, 20, 67), Some(2));
    assert_eq!(menu_entry_at(&service, 20, 142), Some(0));
    assert_eq!(menu_entry_at(&service, 20, 43), None);
}

#[test]
fn tapping_an_entry_selects_it_then_a_second_tap_activates_it() {
    let mut service = Service::new();

    assert_eq!(
        menu_tap_command(&service, 120, 110),
        Some(NavCommand::Digit(2))
    );
    service.apply(NavCommand::Digit(2));
    assert_eq!(menu_tap_command(&service, 120, 110), Some(NavCommand::Send));
}

#[test]
fn menu_taps_are_disabled_on_detail_screens_and_outside_terminal_content() {
    let mut service = Service::new();
    open_page(&mut service, 1);

    assert!(service.current_detail().is_some());
    assert_eq!(menu_entry_at(&service, 120, 94), None);
    assert_eq!(menu_tap_command(&service, 120, 94), None);
    assert_eq!(menu_entry_at(&Service::new(), 2, 94), None);
    assert_eq!(menu_entry_at(&Service::new(), 318, 94), None);
}
