use gpe_3615_gcho::{ContentBundle, DetailId, NavCommand, PageId, Service};

fn open_page(service: &mut Service, digit: u8) {
    service.apply(NavCommand::Digit(digit));
    service.apply(NavCommand::Send);
}

#[test]
fn rencontres_is_a_real_messagerie_detail() {
    let mut service = Service::new();
    open_page(&mut service, 2);
    assert_eq!(service.current_page(), PageId::Messagerie);

    open_page(&mut service, 3);

    assert_eq!(service.current_detail(), Some(DetailId::Rencontres));
    assert_eq!(service.notice(), None);
}

#[test]
fn rencontres_has_bundled_editorial_content() {
    let bundle = ContentBundle::load_bundled();

    assert!(bundle.rencontres.messages.len() >= 4);
    assert!(
        bundle
            .rencontres
            .messages
            .iter()
            .all(|message| message.category == "rencontres")
    );
}
