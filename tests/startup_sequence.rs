use gpe_3615_gcho::{BOOT_DURATION_SECONDS, BRANDING_SPLASH_DURATION_SECONDS, StartupStage, startup_stage};

#[test]
fn startup_sequence_progresses_from_boot_to_branding_to_service() {
    assert_eq!(startup_stage(0.0), StartupStage::Boot);
    assert_eq!(startup_stage(BOOT_DURATION_SECONDS - 0.01), StartupStage::Boot);
    assert_eq!(startup_stage(BOOT_DURATION_SECONDS), StartupStage::Branding);
    assert_eq!(
        startup_stage(BOOT_DURATION_SECONDS + BRANDING_SPLASH_DURATION_SECONDS - 0.01),
        StartupStage::Branding
    );
    assert_eq!(
        startup_stage(BOOT_DURATION_SECONDS + BRANDING_SPLASH_DURATION_SECONDS),
        StartupStage::Service
    );
}
