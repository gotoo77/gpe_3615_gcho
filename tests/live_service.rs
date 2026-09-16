use gpe_3615_gcho::{LiveServicePulse, selection_hint};

#[test]
fn public_service_banner_rotates_deterministically_and_wraps() {
    let mut pulse = LiveServicePulse::new();

    assert_eq!(pulse.banner_index(3), None);

    pulse.advance(8.0);
    assert_eq!(pulse.banner_index(3), Some(0));

    pulse.advance(8.0);
    assert_eq!(pulse.banner_index(3), Some(1));

    pulse.advance(8.0);
    assert_eq!(pulse.banner_index(3), Some(2));

    pulse.advance(8.0);
    assert_eq!(pulse.banner_index(3), None);
}

#[test]
fn session_status_events_cycle_without_runtime_network() {
    let mut pulse = LiveServicePulse::new();

    assert_eq!(pulse.status(), "RESEAU GCHO : OUVERT");

    pulse.advance(12.0);
    assert_eq!(pulse.status(), "1 NOUVEAU MESSAGE");

    pulse.advance(12.0);
    assert_eq!(pulse.status(), "NOEUD 7 : ACTIVITE DETECTEE");

    pulse.advance(12.0);
    assert_eq!(pulse.status(), "ALERTE : FORMULAIRE NON RECU");

    pulse.advance(12.0);
    assert_eq!(pulse.status(), "RESEAU GCHO : OUVERT");
}

#[test]
fn directional_selection_has_an_explicit_persistent_hint() {
    assert_eq!(selection_hint(None), "FLECHES HAUT/BAS : NAVIGUER");
    assert_eq!(selection_hint(Some(3)), "SELECTION : 3 / ENVOI");
}
