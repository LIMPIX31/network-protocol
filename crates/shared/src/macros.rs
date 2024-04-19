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

#[macro_export]
macro_rules! report {
  ($expr:expr, $($tt:tt)*) => {
    $expr.map_err(|err| {
	    ::tracing::error!(?err, $($tt)*);
	    err
    })
  };
}

#[macro_export]
macro_rules! reflect_get {
  ($target:expr, $prop:literal) => {
    ::js_sys::Reflect::get($target, &js!($prop))
  };
	($target:expr, $prop:expr) => {
    ::js_sys::Reflect::get($target, $prop)
  };
}

#[macro_export]
macro_rules! set_js_closure {
  ($closure:expr, $ident:ident => $expr:expr) => {
    let __closure = ::net_shared::js_closure!($closure);
    let $ident = __closure.as_ref().unchecked_ref();
	  $expr;
	  __closure.forget();
  };
}
