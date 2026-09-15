use gpe_3615_gcho::{NavCommand, PageId, Service};

#[test]
fn required_pages_are_available() {
    let pages = Service::available_pages();
    for expected in [
        PageId::Accueil,
        PageId::Arcade,
        PageId::Messagerie,
        PageId::Infos,
        PageId::Gpe,
        PageId::Noeud7,
        PageId::Aide,
    ] {
        assert!(pages.contains(&expected), "missing page: {expected:?}");
    }
}

#[test]
fn digit_then_send_opens_requested_service() {
    let mut service = Service::new();
    service.apply(NavCommand::Digit(3));
    assert_eq!(service.pending_digit(), Some(3));
    service.apply(NavCommand::Send);
    assert_eq!(service.current_page(), PageId::Infos);
    assert_eq!(service.pending_digit(), None);
}

#[test]
fn correction_clears_pending_selection() {
    let mut service = Service::new();
    service.apply(NavCommand::Digit(2));
    service.apply(NavCommand::Correction);
    assert_eq!(service.current_page(), PageId::Accueil);
    assert_eq!(service.pending_digit(), None);
}

#[test]
fn return_and_summary_follow_minitel_navigation() {
    let mut service = Service::new();
    service.apply(NavCommand::Digit(1));
    service.apply(NavCommand::Send);
    assert_eq!(service.current_page(), PageId::Arcade);

    service.apply(NavCommand::Return);
    assert_eq!(service.current_page(), PageId::Accueil);

    service.apply(NavCommand::Digit(5));
    service.apply(NavCommand::Send);
    assert_eq!(service.current_page(), PageId::Noeud7);
    service.apply(NavCommand::Summary);
    assert_eq!(service.current_page(), PageId::Accueil);
}
