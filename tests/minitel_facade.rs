use gpe_3615_gcho::{FunctionKey, NavCommand, PageId, Service, function_key_at, function_keys};

#[test]
fn minitel_function_keys_are_visible_and_hit_testable() {
    let keys = function_keys();
    assert_eq!(keys.len(), 5);
    assert_eq!(keys[0].key, FunctionKey::Summary);
    assert_eq!(keys[1].key, FunctionKey::Return);
    assert_eq!(keys[2].key, FunctionKey::Correction);
    assert_eq!(keys[3].key, FunctionKey::Guide);
    assert_eq!(keys[4].key, FunctionKey::Send);

    for spec in keys {
        let x = spec.x + spec.width / 2;
        let y = spec.y + spec.height / 2;
        assert_eq!(function_key_at(x, y), Some(spec.key));
    }

    assert_eq!(function_key_at(0, 0), None);
}

#[test]
fn guide_is_a_real_navigation_command() {
    let mut service = Service::new();
    service.apply(NavCommand::Guide);
    assert_eq!(service.current_page(), PageId::Aide);
}

#[test]
fn each_function_key_maps_to_the_expected_command() {
    assert_eq!(FunctionKey::Summary.command(), NavCommand::Summary);
    assert_eq!(FunctionKey::Return.command(), NavCommand::Return);
    assert_eq!(FunctionKey::Correction.command(), NavCommand::Correction);
    assert_eq!(FunctionKey::Guide.command(), NavCommand::Guide);
    assert_eq!(FunctionKey::Send.command(), NavCommand::Send);
}
