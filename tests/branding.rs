use gpe_3615_gcho::{ACCUEIL_LOGO_RECT, decode_minitel_logo};

#[test]
fn bundled_minitel_logo_decodes_with_expected_source_dimensions() {
    let logo = decode_minitel_logo().expect("bundled Minitel logo should decode");

    assert_eq!(logo.width(), 2172);
    assert_eq!(logo.height(), 724);
}

#[test]
fn accueil_logo_rect_fits_the_terminal_without_touching_navigation_regions() {
    assert_eq!(ACCUEIL_LOGO_RECT.x, 85);
    assert_eq!(ACCUEIL_LOGO_RECT.y, 28);
    assert_eq!(ACCUEIL_LOGO_RECT.width, 150);
    assert_eq!(ACCUEIL_LOGO_RECT.height, 50);
}
