use gpe_3615_gcho::{NavCommand, PageId, Service, menu_entry_at, menu_tap_command};

#[test]
fn accueil_rows_are_touchable_in_the_visible_branded_layout() {
    let service = Service::new();

    assert_eq!(menu_entry_at(&service, 20, 91), Some(1));
    assert_eq!(menu_entry_at(&service, 20, 101), Some(2));
    assert_eq!(menu_entry_at(&service, 20, 151), Some(0));
    assert_eq!(menu_entry_at(&service, 20, 82), None);
    assert_eq!(menu_entry_at(&service, 20, 164), None);
}

#[test]
fn normal_service_rows_are_touchable_at_their_rendered_positions() {
    let mut service = Service::new();
    service.apply(NavCommand::Digit(2));
    service.apply(NavCommand::Send);
    assert_eq!(service.current_page(), PageId::Messagerie);

    assert_eq!(menu_entry_at(&service, 20, 52), Some(1));
    assert_eq!(menu_entry_at(&service, 20, 67), Some(2));
    assert_eq!(menu_entry_at(&service, 20, 112), Some(5));
    assert_eq!(menu_entry_at(&service, 20, 43), None);
}

#[test]
fn tapping_an_entry_selects_it_then_a_second_tap_activates_it() {
    let mut service = Service::new();

    assert_eq!(menu_tap_command(&service, 20, 101), Some(NavCommand::Digit(2)));
    service.apply(NavCommand::Digit(2));
    assert_eq!(menu_tap_command(&service, 20, 101), Some(NavCommand::Send));
}

#[test]
fn menu_taps_are_disabled_on_detail_screens_and_outside_terminal_content() {
    let mut service = Service::new();
    service.apply(NavCommand::Digit(1));
    service.apply(NavCommand::Send);
    service.apply(NavCommand::Digit(1));
    service.apply(NavCommand::Send);

    assert!(service.current_detail().is_some());
    assert_eq!(menu_entry_at(&service, 20, 52), None);
    assert_eq!(menu_tap_command(&service, 20, 52), None);
    assert_eq!(menu_entry_at(&Service::new(), 2, 91), None);
    assert_eq!(menu_entry_at(&Service::new(), 318, 91), None);
}
