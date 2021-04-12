use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use indoc::{formatdoc, writedoc};
use proc_macro2::{Ident, Span};
use svd_parser::{Access, Field, Register, RegisterProperties};

use crate::field::_Field;
use crate::utils::{array_names, build_ident, indent};

#[derive(Clone)]
pub(super) struct _Register<'a> {
    pub(super) name: Ident,
    description: &'a Option<String>,
    offset: u64,
    value_size: u32,
    reset_mask: u64,
    reset_value: u64,
    fields: Vec<_Field<'a>>,
}

impl<'a> _Register<'a> {
    pub(super) fn build_all(
        register: &'a Register,
        registers: &HashMap<String, &'a Register>,
        defaults: RegisterProperties,
    ) -> Vec<_Register<'a>> {
        let _register = _Register::build(register, registers, defaults);
        match register {
            Register::Single(_) => vec![_Register {
                name: build_ident(register.name.to_lowercase()),
                .._register
            }],
            Register::Array(_, dim) => {
                let mut registers = Vec::new();
                let mut offset = _register.offset;
                for name in array_names(&register.name, dim) {
                    registers.push(_Register {
                        name: build_ident(name),
                        offset,
                        fields: _register.fields.clone(),
                        .._register
                    });
                    offset = offset + dim.dim_increment as u64;
                }
                registers
            }
        }
    }

    pub(super) fn build(
        register: &'a Register,
        registers: &HashMap<String, &'a Register>,
        defaults: RegisterProperties,
    ) -> _Register<'a> {
        let access = register.access(registers).or(defaults.access);
        let children = register
            .fields(registers)
            .expect("Register should not be empty");
        let fields = Self::collect_fields(children);
        let child_fields = children
            .iter()
            .flat_map(|field| _Field::build_all(field, &fields, access))
            .collect();
        _Register {
            name: Ident::new("_", Span::call_site()), // placeholder
            description: &register.description,
            offset: register.address_offset as u64,
            value_size: register
                .size(registers)
                .or(defaults.size)
                .expect("Default size must be specified"),
            reset_mask: register
                .reset_mask(registers)
                .or(defaults.reset_mask)
                .expect("Default reset mask must be specified"),
            reset_value: register
                .reset_value(registers)
                .or(defaults.reset_value)
                .expect("Default reset value must be specified"),
            fields: child_fields,
        }
    }

    fn collect_fields(fields: &Vec<Field>) -> HashMap<String, &Field> {
        let mut map = HashMap::<String, &Field>::new();
        for field in fields {
            match field {
                Field::Single(single) => {
                    map.insert(single.name.clone(), field);
                }
                Field::Array(array, dim) => {
                    for name in array_names(&array.name, dim) {
                        map.insert(name, field);
                    }
                }
            }
        }
        map
    }
}

impl<'a> Display for _Register<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(description) = self.description {
            write!(
                f,
                "#![doc = \"{description}\"]\n\n",
                description = description
            )?;
        }
        write!(f, "use ral::register;\n\nregister! {{\n")?;
        if let Some(description) = self.description {
            f.write_str(&indent(
                format!("#[doc = \"{description}\"]\n", description = description),
                1,
            ))?;
        }
        f.write_str(&indent(
            formatdoc!(
                "
                {name} {{
                    offset: {offset:#X},
                    value_size: {value_size},
                    reset_mask: {reset_mask:#X},
                    reset_value: {reset_value:#X},
                    fields: {{
                ",
                name = self.name,
                offset = self.offset,
                value_size = self.value_size,
                reset_mask = self.reset_mask,
                reset_value = self.reset_value,
            ),
            1,
        ))?;
        for field in &self.fields {
            write!(f, "{},\n", field)?;
        }
        writedoc!(
            f,
            "
                    }}
                }}
            }}"
        )
    }
}

trait DerivedRegister<'a> {
    fn size(&self, registers: &HashMap<String, &'a Register>) -> Option<u32>;

    fn reset_mask(&self, registers: &HashMap<String, &'a Register>) -> Option<u64>;

    fn reset_value(&self, registers: &HashMap<String, &'a Register>) -> Option<u64>;

    fn access(&self, registers: &HashMap<String, &'a Register>) -> Option<Access>;

    fn fields(&'a self, registers: &HashMap<String, &'a Register>) -> Option<&'a Vec<Field>>;
}

impl<'a> DerivedRegister<'a> for Register {
    fn size(&self, registers: &HashMap<String, &'a Register>) -> Option<u32> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| registers.get(name))
            .and_then(|&r| r.size(registers));
        self.size.or(derived)
    }

    fn reset_mask(&self, registers: &HashMap<String, &'a Register>) -> Option<u64> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| registers.get(name))
            .and_then(|&r| r.reset_mask(registers));
        self.reset_mask.or(derived)
    }

    fn reset_value(&self, registers: &HashMap<String, &'a Register>) -> Option<u64> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| registers.get(name))
            .and_then(|&r| r.reset_value(registers));
        self.reset_value.or(derived)
    }

    fn access(&self, registers: &HashMap<String, &'a Register>) -> Option<Access> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| registers.get(name))
            .and_then(|&r| r.access(registers));
        self.access.or(derived)
    }

    fn fields(&'a self, registers: &HashMap<String, &'a Register>) -> Option<&'a Vec<Field>> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| registers.get(name))
            .and_then(|&r| r.fields(registers));
        self.fields.as_ref().or(derived)
    }
}
