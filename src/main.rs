use gotoo_pixel_engine::run;
use gpe_3615_gcho::{GchoApp, TerminalTiming, engine_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(engine_config(), GchoApp::with_timing(terminal_timing()))?;
    Ok(())
}

fn terminal_timing() -> TerminalTiming {
    let defaults = TerminalTiming::default();
    let baud = std::env::var("GCHO_BAUD")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(defaults.baud());
    let response_delay_ms = std::env::var("GCHO_RESPONSE_MS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(defaults.response_delay_ms());

    TerminalTiming::try_from_baud(baud, response_delay_ms).unwrap_or(defaults)
}
