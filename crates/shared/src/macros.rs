#[macro_export]
macro_rules! js_await {
	($expr:expr) => {
		::wasm_bindgen_futures::JsFuture::from($expr).await
	};
}

#[macro_export]
macro_rules! js {
	($expr:expr) => {
		::wasm_bindgen::JsValue::from($expr)
	};
}
