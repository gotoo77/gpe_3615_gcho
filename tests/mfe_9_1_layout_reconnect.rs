use gpe_3615_gcho::{LIVE_STATUS_TOP_Y, NavCommand, PageId, Service, choice_prompt_y};

fn open_page(service: &mut Service, digit: u8) {
    service.apply(NavCommand::Digit(digit));
    service.apply(NavCommand::Send);
}

#[test]
fn choice_prompt_never_collides_with_live_status_and_is_hidden_on_accueil() {
    assert_eq!(choice_prompt_y(PageId::Accueil), None);

    for page in [
        PageId::Arcade,
        PageId::Messagerie,
        PageId::Infos,
        PageId::Gpe,
        PageId::Noeud7,
        PageId::Aide,
    ] {
        let y = choice_prompt_y(page).expect("non-accueil pages keep the choice prompt");
        assert!(y + 8 < LIVE_STATUS_TOP_Y);
    }
}

#[test]
fn aide_can_request_a_full_connection_replay() {
    let mut service = Service::new();

    open_page(&mut service, 6);
    assert_eq!(service.current_page(), PageId::Services);

    open_page(&mut service, 6);
    assert_eq!(service.current_page(), PageId::Aide);

    open_page(&mut service, 2);

    assert!(service.take_reconnect_requested());
    assert_eq!(service.current_page(), PageId::Accueil);
    assert_eq!(service.current_detail(), None);
    assert_eq!(service.pending_digit(), None);
    assert!(!service.take_reconnect_requested());
}
