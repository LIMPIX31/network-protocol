use std::panic;
use tracing::info;

use wasm_bindgen::prelude::wasm_bindgen;

mod telemetry;

#[wasm_bindgen(start)]
fn start() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	telemetry::setup();
	info!("Hello");
}
