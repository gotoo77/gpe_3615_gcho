use gpe_3615_gcho::{
    FUNCTION_KEYS_TOP_Y, LIVE_STATUS_HEIGHT, LIVE_STATUS_TOP_Y, NODE7_FILE_FINAL_Y, NOTICE_HEIGHT,
    NOTICE_TOP_Y, PageId, choice_prompt_y, function_keys,
};

const TEXT_HEIGHT: i32 = 7;

#[test]
fn notices_and_detail_text_stay_above_the_live_status_band() {
    assert!(NOTICE_TOP_Y + NOTICE_HEIGHT as i32 <= LIVE_STATUS_TOP_Y);
    assert!(NODE7_FILE_FINAL_Y + TEXT_HEIGHT < LIVE_STATUS_TOP_Y);
}

#[test]
fn prompts_stay_above_the_live_status_band() {
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
        assert!(y + TEXT_HEIGHT < LIVE_STATUS_TOP_Y);
    }
}

#[test]
fn live_status_and_function_keys_have_separate_vertical_bands() {
    assert!(LIVE_STATUS_TOP_Y + LIVE_STATUS_HEIGHT <= FUNCTION_KEYS_TOP_Y);
    assert!(
        function_keys()
            .iter()
            .all(|spec| spec.y >= FUNCTION_KEYS_TOP_Y)
    );
}
