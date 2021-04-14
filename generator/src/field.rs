use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use svd_parser::{Access, Field};

use crate::overrides::FieldOverrides;
use crate::utils::{array_names, build_ident};

#[derive(Clone)]
pub(super) struct _Field<'a> {
    name: String,
    description: Option<&'a String>,
    offset: u32,
    width: u32,
    access: Option<Access>,
    ty: &'a str,
}

impl<'a> _Field<'a> {
    pub(super) fn build_all(
        field: &'a Field,
        fields: &HashMap<String, &Field>,
        overrides: Option<&'a HashMap<String, FieldOverrides>>,
    ) -> Vec<_Field<'a>> {
        let _field = _Field::build(field, fields, overrides);
        match field {
            Field::Single(_) => vec![_field],
            Field::Array(_, dim) => {
                let mut fields = Vec::new();
                let mut offset = _field.offset;
                for name in array_names(&_field.name, dim) {
                    fields.push(_Field {
                        name,
                        offset,
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
        overrides: Option<&'a HashMap<String, FieldOverrides>>,
    ) -> _Field<'a> {
        let overrides = field.overrides(overrides);
        let range = field.bit_range;
        let width = range.width;
        _Field {
            name: field.name(overrides),
            description: field.description(overrides),
            offset: range.offset,
            width,
            access: field.access(fields),
            ty: field.ty(overrides),
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
        write_access!(f, self.access, indent);
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
    fn name(&self, overrides: Option<&'a FieldOverrides>) -> String;

    fn description(&'a self, overrides: Option<&'a FieldOverrides>) -> Option<&'a String>;

    fn ty(&'a self, overrides: Option<&'a FieldOverrides>) -> &'a str;

    fn access(&self, fields: &HashMap<String, &'a Field>) -> Option<Access>;

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, FieldOverrides>>,
    ) -> Option<&'a FieldOverrides>;
}

impl<'a> DerivedField<'a> for Field {
    fn name(&self, overrides: Option<&'a FieldOverrides>) -> String {
        build_ident(
            overrides
                .and_then(|overrides| overrides.name.as_ref())
                .unwrap_or(&self.name),
        )
    }

    fn description(&'a self, overrides: Option<&'a FieldOverrides>) -> Option<&'a String> {
        overrides
            .and_then(|overrides| overrides.description.as_ref())
            .or(self.description.as_ref())
    }

    fn ty(&'a self, overrides: Option<&'a FieldOverrides>) -> &'a str {
        overrides
            .and_then(|overrides| overrides.ty.as_ref())
            .map(|ty_name| ty_name.as_str())
            .unwrap_or_else(|| {
                let width = self.bit_range.width;
                match width {
                    1 => "bool",
                    2..=8 => "u8",
                    9..=16 => "u16",
                    17..=32 => "u32",
                    33..=64 => "u64",
                    _ => panic!("Unexpected field width"),
                }
            })
    }

    fn access(&self, fields: &HashMap<String, &'a Field>) -> Option<Access> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| fields.get(name))
            .and_then(|f| f.access(fields));
        self.access.or(derived)
    }

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, FieldOverrides>>,
    ) -> Option<&'a FieldOverrides> {
        overrides!(self, overrides)
    }
}
