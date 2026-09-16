use gpe_3615_gcho::{
    FUNCTION_KEYS_TOP_Y, LIVE_STATUS_HEIGHT, LIVE_STATUS_TOP_Y, NODE7_FILE_FINAL_Y, NOTICE_HEIGHT,
    NOTICE_TOP_Y, PageId, choice_prompt_y, function_keys,
};

const TEXT_HEIGHT: i32 = 7;

fn assert_band_before(top: i32, height: i32, next_top: i32) {
    assert!(top + height <= next_top);
}

fn assert_text_before(y: i32, boundary: i32) {
    assert!(y + TEXT_HEIGHT < boundary);
}

#[test]
fn notices_and_detail_text_stay_above_the_live_status_band() {
    assert_band_before(NOTICE_TOP_Y, NOTICE_HEIGHT as i32, LIVE_STATUS_TOP_Y);
    assert_text_before(NODE7_FILE_FINAL_Y, LIVE_STATUS_TOP_Y);
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
        assert_text_before(y, LIVE_STATUS_TOP_Y);
    }
}

#[test]
fn live_status_and_function_keys_have_separate_vertical_bands() {
    assert_band_before(LIVE_STATUS_TOP_Y, LIVE_STATUS_HEIGHT, FUNCTION_KEYS_TOP_Y);
    assert!(
        function_keys()
            .iter()
            .all(|spec| spec.y >= FUNCTION_KEYS_TOP_Y)
    );
}
