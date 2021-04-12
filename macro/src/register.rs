use syn::{Attribute, braced, Ident, LitInt, LitStr};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::token::{Colon, Comma};

use crate::field::_Fields;
use crate::parse::{attrs_to_meta_map, get_meta};
use crate::spanned::_Spanned;

pub(crate) struct _Register {
    pub(crate) name: Ident,
    pub(crate) description: LitStr,
    pub(crate) offset: _Spanned<u32>,
    pub(crate) value_size: _Spanned<u32>,
    pub(crate) reset_mask: LitInt,
    pub(crate) reset_value: LitInt,
    pub(crate) fields: _Fields,
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
        let mut fields: Result<_Fields> = Err(syn::Error::new(
            name.span(),
            format!("Register fields is not specified"),
        ));
        while !content.is_empty() {
            let field_name: Ident = content.parse()?;
            let _: Colon = content.parse()?;
            match field_name.to_string().as_str() {
                "offset" => offset = content.parse(),
                "value_size" => value_size = content.parse(),
                "reset_mask" => reset_mask = content.parse(),
                "reset_value" => reset_value = content.parse(),
                "fields" => fields = content.parse(),
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
        let value_size = _Spanned::from(value_size).and_then(validate_value_size)?;
        let fields = fields?;
        fields.validate(value_size.value, name.span())?;

        Ok(_Register {
            name,
            description,
            offset: _Spanned::from(offset)?,
            value_size,
            reset_mask: reset_mask?,
            reset_value: reset_value?,
            fields,
        })
    }
}

fn validate_value_size(value_size: _Spanned<u32>) -> Result<_Spanned<u32>> {
    let value = value_size.value;
    if value < 8 || value > 64 || (value & (value - 1)) != 0 {
        Err(syn::Error::new(
            value_size.span(),
            format!(
                "Unsupported register size value {}, must be 8 <= power of 2 <= 64",
                value
            ),
        ))?
    }
    Ok(value_size)
}
