use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use proc_macro2::{Ident, Span};
use quote::format_ident;
use svd_parser::{Access, Field};

use crate::utils::{array_names, build_ident};

#[derive(Clone)]
pub(super) struct _Field<'a> {
    name: Ident,
    description: &'a Option<String>,
    offset: u32,
    width: u32,
    access: Option<Access>,
    ty: Ident,
}

impl<'a> _Field<'a> {
    pub(super) fn build_all(
        field: &'a Field,
        fields: &HashMap<String, &Field>,
        default_access: Option<Access>,
    ) -> Vec<_Field<'a>> {
        let _field = _Field::build(field, fields, default_access);
        match field {
            Field::Single(_) => vec![_Field {
                name: build_ident(field.name.to_lowercase()),
                .._field
            }],
            Field::Array(_, dim) => {
                let mut fields = Vec::new();
                let mut offset = _field.offset;
                for name in array_names(&field.name, dim) {
                    fields.push(_Field {
                        name: build_ident(name),
                        offset,
                        ty: _field.ty.clone(),
                        .._field
                    });
                    offset = offset + dim.dim_increment;
                }
                fields
            }
        }
    }

    pub(super) fn build(
        field: &'a Field,
        fields: &HashMap<String, &Field>,
        default_access: Option<Access>,
    ) -> _Field<'a> {
        let range = field.bit_range;
        let width = range.width;
        _Field {
            name: Ident::new("_", Span::call_site()), //placeholder
            description: &field.description,
            offset: range.offset,
            width,
            access: field.access(fields).or(default_access),
            ty: Self::infer_type(width),
        }
    }

    fn infer_type(width: u32) -> Ident {
        match width {
            1 => format_ident!("bool"),
            2..=8 => format_ident!("u8"),
            9..=16 => format_ident!("u16"),
            17..=32 => format_ident!("u32"),
            33..=64 => format_ident!("u64"),
            _ => panic!("Unexpected field width"),
        }
    }
}

impl<'a> Display for _Field<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = " ".repeat(4 * 3);
        if let Some(description) = self.description {
            write!(
                f,
                "{indent}#[doc = \"{description}\"]\n",
                description = description,
                indent = indent
            )?;
        }
        let access = self.access.and_then(|access| match access {
            Access::ReadOnly => Some("read-only"),
            Access::WriteOnly => Some("write-only"),
            Access::ReadWrite => None,
            Access::WriteOnce => Some("writeOnce"),
            Access::ReadWriteOnce => Some("read-writeOnce"),
        });
        if let Some(access) = access {
            write!(
                f,
                "{indent}#[access = \"{access}\"]\n",
                access = access,
                indent = indent
            )?;
        }
        write!(
            f,
            "{indent}{name}[{offset}:{width}] as {ty}",
            name = self.name,
            offset = self.offset,
            width = self.width,
            ty = self.ty,
            indent = indent
        )
    }
}

trait DerivedField<'a> {
    fn access(&self, fields: &HashMap<String, &'a Field>) -> Option<Access>;
}

impl<'a> DerivedField<'a> for Field {
    fn access(&self, fields: &HashMap<String, &'a Field>) -> Option<Access> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| fields.get(name))
            .and_then(|f| f.access(fields));
        self.access.or(derived)
    }
}
