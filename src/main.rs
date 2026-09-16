use gotoo_pixel_engine::run;
use gpe_3615_gcho::{GchoApp, engine_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(engine_config(), GchoApp::new())?;
    Ok(())
}
