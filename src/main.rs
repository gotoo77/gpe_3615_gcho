use gpe_3615_gcho::{GchoApp, engine_config};
use gotoo_pixel_engine::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(engine_config(), GchoApp::new())?;
    Ok(())
}
