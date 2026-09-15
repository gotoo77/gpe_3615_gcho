use gpe_3615_gcho::{ContentBundle, ContentFile};

const VALID: &str = r#"{
  "generated_at": "2026-09-15T00:00:00Z",
  "messages": [{
    "id": "test-1",
    "category": "test",
    "title": "TEST",
    "body": ["LIGNE 1"]
  }]
}"#;

#[test]
fn valid_editorial_json_parses() {
    let parsed = ContentFile::from_json(VALID).expect("valid content should parse");
    assert_eq!(parsed.messages[0].id, "test-1");
}

#[test]
fn invalid_sources_use_non_empty_fallbacks() {
    let bundle = ContentBundle::from_sources("not-json", "{}", "[]", "");
    assert!(!bundle.service_public.messages.is_empty());
    assert!(!bundle.news.messages.is_empty());
    assert!(!bundle.messages.messages.is_empty());
    assert!(!bundle.secrets.messages.is_empty());
}

#[test]
fn bundled_snapshots_are_all_usable() {
    let bundle = ContentBundle::load_bundled();
    assert!(!bundle.service_public.messages.is_empty());
    assert!(!bundle.news.messages.is_empty());
    assert!(!bundle.messages.messages.is_empty());
    assert!(!bundle.secrets.messages.is_empty());
}
