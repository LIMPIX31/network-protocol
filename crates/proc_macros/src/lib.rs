extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{Error, parse_macro_input};

mod expand;

#[proc_macro]
pub fn js_closure(input: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(input);

	expand::js_closure_expand(&mut input).unwrap_or_else(Error::into_compile_error).into()
}
