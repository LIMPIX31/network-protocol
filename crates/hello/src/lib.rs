use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(data: &[u8]) -> Vec<u8> {
	lz4_flex::compress(data)
}
