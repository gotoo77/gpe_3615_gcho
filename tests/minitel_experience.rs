use gpe_3615_gcho::{DetailId, NavCommand, Service, TerminalTiming, public_service_slogan};

#[test]
fn directional_navigation_selects_entries_and_wraps() {
    let mut service = Service::new();

    service.apply(NavCommand::Next);
    assert_eq!(service.pending_digit(), Some(1));

    service.apply(NavCommand::Next);
    assert_eq!(service.pending_digit(), Some(2));

    service.apply(NavCommand::Previous);
    assert_eq!(service.pending_digit(), Some(1));

    service.apply(NavCommand::Previous);
    assert_eq!(service.pending_digit(), Some(6));
}

#[test]
fn directional_selection_can_be_sent_without_typing_a_digit() {
    let mut service = Service::new();
    service.apply(NavCommand::Next);
    service.apply(NavCommand::Send);

    assert_eq!(service.current_page(), gpe_3615_gcho::PageId::Accueil);
    assert_eq!(service.current_detail(), Some(DetailId::RoyalCommunique));
}

#[test]
fn minitel_timing_converts_baud_to_character_budget_after_response_delay() {
    let timing = TerminalTiming::from_baud(1_200, 180);

    assert_eq!(timing.characters_per_second(), 120);
    assert_eq!(timing.visible_characters(0.17), 0);
    assert_eq!(timing.visible_characters(0.18), 0);
    assert_eq!(timing.visible_characters(0.68), 60);
}

#[test]
fn timing_can_be_overridden_but_rejects_zero_baud() {
    assert!(TerminalTiming::try_from_baud(2_400, 50).is_some());
    assert!(TerminalTiming::try_from_baud(0, 50).is_none());
}

#[test]
fn landing_page_leads_with_the_satirical_public_service_slogan() {
    assert_eq!(
        public_service_slogan(),
        "DITES-NOUS DE QUOI VOUS AVEZ BESOIN. NOUS VOUS DIRONS COMMENT VOUS EN PASSER."
    );
}
