use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const DTS: &str = r#"
type TransportError =
{
	kind: 'js'
	error: any
}
|
{
	kind: 'type_mismatch'
}
"#;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(typescript_type = "TransportError")]
	pub type JsError;
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Js exception: {0:?}")]
	Js(JsValue),

	#[error("Type mismatch")]
	TypeMismatch
}

impl From<JsValue> for Error {
	fn from(value: JsValue) -> Self {
		Self::Js(value)
	}
}

#[wasm_bindgen]
struct JsErrorRepr {
	kind: &'static str,
	value: Option<JsValue>,
}

impl JsErrorRepr {
	fn new(kind: &'static str, value: Option<JsValue>) -> JsErrorRepr {
		JsErrorRepr { kind, value }
	}
}

impl Into<JsValue> for Error {
	fn into(self) -> JsValue {
		match self {
			Self::Js(it) => JsErrorRepr::new("js", Some(it)),
			Self::TypeMismatch => JsErrorRepr::new("type_mismatch", None),
		}
		.into()
	}
}
