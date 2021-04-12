use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use syn::{Attribute, Lit, LitStr, Meta, Result};
use syn::spanned::Spanned;

pub(crate) fn adjust_ident(ident: Ident) -> Ident {
    let string_ident = ident.to_string();
    if string_ident.starts_with("_") {
        Ident::new(string_ident.trim_start_matches("_"), ident.span())
    } else {
        ident
    }
}

pub(crate) fn attrs_to_meta_map(attrs: Vec<Attribute>) -> Result<HashMap<String, Meta>> {
    let attrs_iter = attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::<Meta>::ok);
    let mut attrs = HashMap::new();
    for attr in attrs_iter {
        let name = attr
            .path()
            .get_ident()
            .ok_or(syn::Error::new(
                attr.path().span(),
                "attribute path is not identifier",
            ))?
            .to_string();
        attrs.insert(name, attr);
    }
    Ok(attrs)
}

pub(crate) fn get_meta(
    attr_name: &str,
    attrs: &mut HashMap<String, Meta>,
    span: Span,
) -> Result<LitStr> {
    attrs
        .remove(attr_name)
        .and_then(|meta| {
            if let Meta::NameValue(attr) = meta {
                Some(attr.lit)
            } else {
                None
            }
        })
        .and_then(|lit| {
            if let Lit::Str(value) = lit {
                Some(value)
            } else {
                None
            }
        })
        .ok_or(syn::Error::new(
            span,
            format!("{} attribute is not specified", attr_name),
        ))
}
