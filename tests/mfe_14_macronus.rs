use gpe_3615_gcho::{DetailId, NavCommand, PageId, Service};

fn activate(service: &mut Service, digit: u8) {
    service.apply(NavCommand::Digit(digit));
    service.apply(NavCommand::Send);
}

#[test]
fn accueil_is_the_new_royal_service_home() {
    let service = Service::new();
    let labels: Vec<_> = service.entries().iter().map(|entry| entry.label).collect();

    assert_eq!(service.current_page(), PageId::Accueil);
    assert_eq!(
        &labels[..6],
        &[
            "COMMUNIQUE ROYAL",
            "ALERTE CANICULE",
            "MESSAGES DU JOUR",
            "PORTRAITS PIXELISES",
            "METEO DU ROYAUME",
            "SUITE        AUTRES SERVICES",
        ]
    );
}

#[test]
fn royal_primary_entries_open_real_detail_screens() {
    let mut service = Service::new();

    activate(&mut service, 1);
    assert_eq!(service.current_detail(), Some(DetailId::RoyalCommunique));

    service.apply(NavCommand::Summary);
    activate(&mut service, 2);
    assert_eq!(service.current_detail(), Some(DetailId::RoyalHeatAlert));
}

#[test]
fn suite_preserves_the_existing_gcho_services() {
    let mut service = Service::new();

    activate(&mut service, 6);
    assert_eq!(service.current_page(), PageId::Services);
    assert_eq!(service.entries()[0].label, "ARCADE       JEUX ET DEMOS");
    assert_eq!(service.entries()[1].label, "MESSAGERIE   DIALOGUER, RENCONTRER");

    activate(&mut service, 1);
    assert_eq!(service.current_page(), PageId::Arcade);

    service.apply(NavCommand::Summary);
    assert_eq!(service.current_page(), PageId::Accueil);
}
