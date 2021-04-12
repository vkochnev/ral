use proc_macro::TokenStream;

use syn::parse_macro_input;

use crate::register::_Register;

mod field;
mod field_type;
mod parse;
mod register;
mod render;
mod spanned;

#[proc_macro]
pub fn register(item: TokenStream) -> TokenStream {
    render::render_register_with_uses(parse_macro_input!(item as _Register))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}

#[proc_macro]
pub fn register_uses(_: TokenStream) -> TokenStream {
    TokenStream::from(render::render_uses())
}

#[proc_macro]
pub fn register_definition(item: TokenStream) -> TokenStream {
    render::render_register(parse_macro_input!(item as _Register))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}
