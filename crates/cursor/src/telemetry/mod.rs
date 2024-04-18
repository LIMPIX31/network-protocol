#![allow(unused)] // TODO: remove

use std::{
	collections::HashMap,
	fmt::{Debug, Display, Formatter},
};

use js_sys::Array;
use tracing::{
	field::{Field, Visit},
	info,
	span::{Attributes, Record},
	Event, Id, Level, Metadata, Subscriber,
};
use wasm_bindgen::JsValue;
use web_sys::console;

struct DisplayLevel(Level);

impl DisplayLevel {
	fn console_log(&self) -> fn(&Array) {
		match self.0 {
			Level::ERROR => console::error,
			Level::WARN => console::warn,
			Level::INFO => console::info,
			Level::DEBUG | Level::TRACE => console::debug,
		}
	}

	fn style(&self) -> JsValue {
		macro_rules! style {
			($color:expr) => {
				JsValue::from_str(concat!(
					"color: black; font-weight: 900; padding: 4px 4px 0 4px; margin-right: 8px; background: ",
					$color,
					";"
				))
			};
		}

		match self.0 {
			Level::ERROR => style!("#ff4d4d"),
			Level::WARN => style!("#dbbb08"),
			Level::INFO => style!("#00d154"),
			Level::DEBUG => style!("#4d88ff"),
			Level::TRACE => style!("#987dff"),
		}
	}
}

impl Display for DisplayLevel {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			Level::ERROR => write!(f, "ERROR"),
			Level::WARN => write!(f, "WARN"),
			Level::INFO => write!(f, "INFO"),
			Level::DEBUG => write!(f, "DEBUG"),
			Level::TRACE => write!(f, "TRACE"),
		}
	}
}

impl From<&Level> for DisplayLevel {
	fn from(value: &Level) -> Self {
		Self(*value)
	}
}

#[derive(Default)]
struct Visitor {
	fields: HashMap<&'static str, Box<str>>,
	message: String,
}

impl Visitor {
	fn format_fields(mut self) -> (String, Vec<JsValue>) {
		let len = self.fields.len() - 1;
		let mut styles = Vec::with_capacity(len * 2 + 1);

		styles.push(JsValue::from_str(""));
		for _ in 0..len {
			styles.push(JsValue::from_str("font-style: italic; font-weight: bold; line-height: 1.6em;"));
			styles.push(JsValue::from_str("font-style: normal; line-height: 1.6em;"));
		}

		let message = self.fields.remove("message").unwrap_or_else(|| Box::from(""));
		let entries = self.fields.into_iter();
		let mut output = String::new();

		output.push_str(&message);

		for (k, v) in entries {
			output.push_str("\n%c");
			output.push_str(k);
			output.push_str("%c = ");
			output.push_str(&v);
		}

		(output, styles)
	}
}

impl Visit for Visitor {
	fn record_str(&mut self, field: &Field, value: &str) {
		self.fields.insert(field.name(), value.into());
	}

	fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
		self.fields.insert(field.name(), format!("{value:?}").into());
	}
}

#[derive(Debug)]
struct WebSubscriber;

impl Subscriber for WebSubscriber {
	fn enabled(&self, metadata: &Metadata<'_>) -> bool {
		true
	}

	fn new_span(&self, span: &Attributes<'_>) -> Id {
		todo!()
	}

	fn record(&self, span: &Id, values: &Record<'_>) {
		todo!()
	}

	fn record_follows_from(&self, span: &Id, follows: &Id) {
		todo!()
	}

	fn event(&self, event: &Event<'_>) {
		let metadata = event.metadata();
		let level: DisplayLevel = metadata.level().into();
		let console_log = level.console_log();

		let mut visitor = Visitor::default();
		event.record(&mut visitor);

		let (fmt, styles) = visitor.format_fields();
		let modpath = metadata.module_path().unwrap_or("<unknown>");
		let modpath_style = "color: gray;";

		let msg = format!("%c{level}%c{modpath} %c{fmt}%c");

		let args = [msg.into(), level.style(), modpath_style.into()].into_iter();

		console_log(&args.chain(styles).collect());
	}

	fn enter(&self, span: &Id) {
		todo!()
	}

	fn exit(&self, span: &Id) {
		todo!()
	}
}

pub(crate) fn setup() {
	tracing::subscriber::set_global_default(WebSubscriber).unwrap();
}
