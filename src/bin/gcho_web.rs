#[cfg(target_arch = "wasm32")]
use gpe_3615_gcho::{GchoApp, engine_config};
#[cfg(target_arch = "wasm32")]
use gotoo_pixel_engine::run;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    run(engine_config(), GchoApp::new()).map_err(|error| JsValue::from_str(&error.to_string()))
}

fn main() {}
