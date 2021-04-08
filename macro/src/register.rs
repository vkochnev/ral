use proc_macro::TokenStream;
use std::collections::HashMap;
use std::str::FromStr;

use proc_macro2::Span;
use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
use syn::spanned::Spanned;
use syn::token::{As, Colon, Comma};
use syn::{
    braced, bracketed, parse_macro_input, Attribute, Ident, Lit, LitInt, LitStr, Meta, TypePath,
};

mod render;

struct _Field {
    name: Ident,
    description: LitStr,
    ty: TypePath,
    offset: LitInt,
    size: LitInt,
    access: Option<LitStr>,
}

struct _Register {
    name: Ident,
    description: LitStr,
    offset: LitInt,
    value_size: LitInt,
    reset_mask: LitInt,
    reset_value: LitInt,
    fields: Vec<_Field>,
}

impl Parse for _Register {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let name: Ident = input.parse()?;
        let content;
        let _ = braced!(content in input);
        attrs.extend(content.call(Attribute::parse_inner)?);
        let mut attrs = attrs_to_meta_map(attrs)?;
        let description = get_meta("doc", &mut attrs, name.span())?;
        if !attrs.is_empty() {
            Err(syn::Error::new(
                name.span(),
                format!("Unexpected attributes specified, only 'doc' attribute expected"),
            ))?
        }
        let mut offset = Err(syn::Error::new(
            name.span(),
            format!("Register offset is not specified"),
        ));
        let mut value_size = Err(syn::Error::new(
            name.span(),
            format!("Register value size is not specified"),
        ));
        let mut reset_mask = Err(syn::Error::new(
            name.span(),
            format!("Register reset mask is not specified"),
        ));
        let mut reset_value = Err(syn::Error::new(
            name.span(),
            format!("Register reset value is not specified"),
        ));
        let mut fields = Vec::new();
        while !content.is_empty() {
            let field_name: Ident = content.parse()?;
            let _: Colon = content.parse()?;
            match field_name.to_string().as_str() {
                "offset" => offset = content.parse(),
                "value_size" => value_size = content.parse(),
                "reset_mask" => reset_mask = content.parse(),
                "reset_value" => reset_value = content.parse(),
                "fields" => parse_fields(&mut fields, &content)?,
                field => Err(syn::Error::new(
                    field_name.span(),
                    format!("Found unexpected field {}", field),
                ))?,
            }
            if content.is_empty() {
                break;
            }
            let _: Comma = content.parse()?;
        }
        let offset = offset?;
        let value_size: LitInt = value_size?;
        let register_size = u32::from_str(value_size.base10_digits()).map_err(|err| {
            syn::Error::new(
                value_size.span(),
                format!("Failed to parse register size: {}", err),
            )
        })?;
        if register_size < 8 || register_size > 64 || (register_size & (register_size - 1)) != 0 {
            Err(syn::Error::new(
                value_size.span(),
                format!(
                    "Unsupported register size value {}, must be 8 <= power of 2 <= 64",
                    register_size
                ),
            ))?
        }
        let reset_mask = reset_mask?;
        let reset_value = reset_value?;
        if fields.is_empty() {
            Err(syn::Error::new(
                name.span(),
                "At least one field must be specified",
            ))?
        }

        Ok(_Register {
            name,
            description,
            offset,
            value_size,
            reset_mask,
            reset_value,
            fields,
        })
    }
}

fn parse_fields(fields: &mut Vec<_Field>, braced_content: &ParseBuffer) -> Result<()> {
    let content;
    let _ = braced!(content in braced_content);
    while !content.is_empty() {
        fields.push(parse_field(&content)?);
        if content.is_empty() {
            break;
        }
        let _: Comma = content.parse()?;
    }
    Ok(())
}

fn parse_field(content: &ParseBuffer) -> Result<_Field> {
    let attrs = content.call(Attribute::parse_outer)?;
    let mut attrs = attrs_to_meta_map(attrs)?;
    let name: Ident = content.parse()?;
    let description = get_meta("doc", &mut attrs, name.span())?;
    let range;
    let _ = bracketed!(range in content);
    let offset = range.parse()?;
    let _: Colon = range.parse()?;
    let size = range.parse()?;
    let _: As = content.parse()?;
    let ty = content.parse()?;
    let access = get_meta("access", &mut attrs, name.span()).ok();
    if !attrs.is_empty() {
        Err(syn::Error::new(
            name.span(),
            format!("Unexpected attributes specified, ony 'doc' and 'access' expected"),
        ))?
    }
    Ok(_Field {
        name,
        description,
        ty,
        offset,
        size,
        access,
    })
}

fn attrs_to_meta_map(attrs: Vec<Attribute>) -> Result<HashMap<String, Meta>> {
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

fn get_meta(attr_name: &str, attrs: &mut HashMap<String, Meta>, span: Span) -> Result<LitStr> {
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

pub(crate) fn register(item: TokenStream) -> TokenStream {
    render::render_register(parse_macro_input!(item as _Register))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}

pub(crate) fn register_uses(_: TokenStream) -> TokenStream {
    TokenStream::from(render::render_uses())
}

pub(crate) fn register_definition(item: TokenStream) -> TokenStream {
    render::render_definition(parse_macro_input!(item as _Register))
        .map(TokenStream::from)
        .unwrap_or_else(|err| TokenStream::from(err.to_compile_error()))
}
