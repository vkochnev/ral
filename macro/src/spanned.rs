use std::fmt::Display;
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{LitInt, Result};

pub(super) struct _Spanned<T> {
    pub(super) value: T,
    pub(super) lit: LitInt,
}

impl<T> ToTokens for _Spanned<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lit.to_tokens(tokens)
    }
}

impl<T: FromStr> _Spanned<T>
where
    <T as FromStr>::Err: Display,
{
    pub(super) fn from(lit: Result<LitInt>) -> Result<_Spanned<T>> {
        let lit = lit?;

        T::from_str(lit.base10_digits())
            .map_err(|err| syn::Error::new(lit.span(), format!("Failed to parse value: {}", err)))
            .map(|value| _Spanned { value, lit })
    }
}
