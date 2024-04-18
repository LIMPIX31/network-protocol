use std::panic;

use wasm_bindgen::prelude::wasm_bindgen;

mod pp;
mod telemetry;

pub use pp::*;

#[wasm_bindgen]
pub fn setup() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	telemetry::setup();
}
