use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Underscore, Error, Expr, ExprClosure, Pat, Result, Token};

fn expr_closure(input: &mut Expr) -> Result<&mut ExprClosure> {
	match input {
		Expr::Closure(it) => Ok(it),
		Expr::Paren(it) => expr_closure(&mut it.expr),
		Expr::Group(it) => expr_closure(&mut it.expr),
		it => Err(Error::new(it.span(), "Closure expected")),
	}
}

pub fn js_closure_expand(input: &mut Expr) -> Result<TokenStream> {
	let closure = expr_closure(input)?;
	let underscorize = |pat: &Pat| Underscore { spans: [pat.span()] };
	let undersocres: Punctuated<Token![_], Token![,]> = closure.inputs.iter().map(underscorize).collect();

	Ok(quote! {
		::wasm_bindgen::closure::Closure::<dyn FnMut(#undersocres)>::new(#closure)
	})
}
