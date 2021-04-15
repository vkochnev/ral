//! Provides macro with DSL describing register
//! For documentation refer to [ral](https://docs.rs/ral) crate
use proc_macro::TokenStream;

use syn::parse_macro_input;

use crate::register::{_Register, _RegisterWithUses};
use crate::uses::_Uses;

mod field;
mod field_type;
mod parse;
mod register;
mod render;
mod spanned;
mod uses;

/// Macro expanding into register definition and required uses
#[proc_macro]
pub fn register(item: TokenStream) -> TokenStream {
    render::render_register_with_uses(parse_macro_input!(item as _RegisterWithUses))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}

/// Macro expanding into required uses, might be useful when multiple registers to be defined in the same module
#[proc_macro]
pub fn register_uses(item: TokenStream) -> TokenStream {
    TokenStream::from(render::render_uses(parse_macro_input!(item as _Uses)))
}

/// Macro expanding into register definition, might be useful when multiple registers to be defined in the same module
#[proc_macro]
pub fn register_definition(item: TokenStream) -> TokenStream {
    render::render_register(parse_macro_input!(item as _Register))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}
