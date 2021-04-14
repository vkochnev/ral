use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{As, Colon, Comma};
use syn::{braced, bracketed, Attribute, LitStr, Result};

use crate::field_type::_FieldType;
use crate::parse::{adjust_ident, attrs_to_meta_map, get_meta};
use crate::spanned::_Spanned;

pub(super) struct _Field {
    pub(super) name: Ident,
    pub(super) description: Option<LitStr>,
    pub(super) ty: _FieldType,
    pub(super) offset: _Spanned<u32>,
    pub(super) width: _Spanned<u32>,
    pub(super) access: Option<LitStr>,
}

impl _Field {
    pub(super) fn validate(&self, value_size: u32) -> Result<()> {
        let width = self.width.value;
        let offset = self.offset.value;
        if width > value_size {
            Err(syn::Error::new(
                self.width.span(),
                format!(
                    "Field width cannot be more than value size [{} > {}]",
                    width, value_size
                ),
            ))?
        }
        if offset >= value_size {
            Err(syn::Error::new(
                self.offset.span(),
                format!(
                    "Field offset must be less than value size [{} >= {}]",
                    offset, value_size
                ),
            ))?
        }
        if offset + width > value_size {
            Err(syn::Error::new(
                self.width.span(),
                format!(
                    "Field offset and width in sum cannot be more than value size [{} + {} > {}]",
                    offset, width, value_size
                ),
            ))?
        }
        Ok(())
    }
}

impl Parse for _Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut attrs = attrs_to_meta_map(attrs)?;
        let name: Ident = adjust_ident(input.parse()?);
        let description = get_meta("doc", &mut attrs, name.span()).ok();
        let range;
        let _ = bracketed!(range in input);
        let offset = _Spanned::from(range.parse())?;
        let _: Colon = range.parse()?;
        let width = _Spanned::from(range.parse())?;
        let _: As = input.parse()?;
        let ty = input.parse()?;
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
            ty: _FieldType::from(ty, width.value, width.span())?,
            offset,
            width,
            access,
        })
    }
}

pub(super) struct _Fields(Option<Vec<_Field>>);

impl _Fields {
    pub(super) fn empty() -> _Fields {
        _Fields(None)
    }

    pub(super) fn validate(&self, value_size: u32) -> Result<()> {
        if let Some(fields) = &self.0 {
            for field in fields {
                field.validate(value_size)?;
            }
        }
        Ok(())
    }
}

impl IntoIterator for _Fields {
    type Item = <Vec<_Field> as IntoIterator>::Item;
    type IntoIter = <Vec<_Field> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        if let Some(fields) = self.0 {
            fields.into_iter()
        } else {
            Vec::<_Field>::new().into_iter()
        }
    }
}

impl Parse for _Fields {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fields = Vec::new();
        let content;
        let _ = braced!(content in input);
        while !content.is_empty() {
            fields.push(content.parse::<_Field>()?);
            if content.is_empty() {
                break;
            }
            let _: Comma = content.parse()?;
        }
        Ok(_Fields(if fields.is_empty() {
            None
        } else {
            Some(fields)
        }))
    }
}
