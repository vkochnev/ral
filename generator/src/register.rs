use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use indoc::{formatdoc, writedoc};
use svd_parser::{Access, Field, Register, RegisterProperties};

use crate::field::_Field;
use crate::overrides::RegisterOverrides;
use crate::utils::{array_names, build_ident, indent};

#[derive(Clone)]
pub(super) struct _Register<'a> {
    pub(super) name: String,
    description: Option<&'a String>,
    access: Option<Access>,
    pub(super) features: Option<&'a Vec<String>>,
    pub(super) uses: Option<&'a Vec<String>>,
    offset: u64,
    value_size: u32,
    reset_mask: u64,
    reset_value: u64,
    fields: Option<Vec<_Field<'a>>>,
}

impl<'a> _Register<'a> {
    pub(super) fn build_all(
        register: &'a Register,
        registers: &HashMap<String, &'a Register>,
        defaults: RegisterProperties,
        overrides: Option<&'a HashMap<String, RegisterOverrides>>,
    ) -> Vec<_Register<'a>> {
        let _register = _Register::build(register, registers, defaults, overrides);
        match register {
            Register::Single(_) => vec![_register],
            Register::Array(_, dim) => {
                let mut registers = Vec::new();
                let mut offset = _register.offset;
                for name in array_names(&_register.name, dim) {
                    registers.push(_Register {
                        name,
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
        overrides: Option<&'a HashMap<String, RegisterOverrides>>,
    ) -> _Register<'a> {
        let overrides = register.overrides(overrides);
        let access = register.access(registers).or(defaults.access);
        let fields = register.fields(registers).map(|children| {
            let fields = Self::collect_fields(children);
            let field_overrides = overrides.and_then(|overrides| overrides.fields.as_ref());
            children
                .iter()
                .flat_map(|field| _Field::build_all(field, &fields, field_overrides))
                .collect()
        });
        _Register {
            name: register.name(overrides),
            description: register.description(overrides),
            access,
            features: overrides.and_then(|overrides| overrides.features.as_ref()),
            uses: overrides.and_then(|overrides| overrides.uses.as_ref()),
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
            fields,
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
        if let Some(uses) = self.uses {
            for use_item in uses {
                f.write_str(&indent(
                    format!("use {use_item};\n", use_item = use_item),
                    1,
                ))?;
            }
            f.write_str("\n")?;
        }
        if let Some(description) = self.description {
            f.write_str(&indent(
                format!("#[doc = \"{description}\"]\n", description = description),
                1,
            ))?;
        }
        write_access!(f, self.access, " ".repeat(4));
        f.write_str(&indent(
            formatdoc!(
                "
                {name} {{
                    offset: {offset:#X},
                    value_size: {value_size},
                    reset_mask: {reset_mask:#X},
                    reset_value: {reset_value:#X},
                ",
                name = build_ident(&self.name),
                offset = self.offset,
                value_size = self.value_size,
                reset_mask = self.reset_mask,
                reset_value = self.reset_value,
            ),
            1,
        ))?;
        if let Some(fields) = &self.fields {
            f.write_str(&indent(String::from("fields: {\n"), 2))?;
            for field in fields {
                write!(f, "{},\n", field)?;
            }
            f.write_str(&indent(String::from("}\n"), 2))?;
        }
        writedoc!(
            f,
            "
                }}
            }}"
        )
    }
}

trait DerivedRegister<'a> {
    fn name(&self, overrides: Option<&'a RegisterOverrides>) -> String;

    fn description(&'a self, overrides: Option<&'a RegisterOverrides>) -> Option<&'a String>;

    fn size(&self, registers: &HashMap<String, &'a Register>) -> Option<u32>;

    fn reset_mask(&self, registers: &HashMap<String, &'a Register>) -> Option<u64>;

    fn reset_value(&self, registers: &HashMap<String, &'a Register>) -> Option<u64>;

    fn access(&self, registers: &HashMap<String, &'a Register>) -> Option<Access>;

    fn fields(&'a self, registers: &HashMap<String, &'a Register>) -> Option<&'a Vec<Field>>;

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, RegisterOverrides>>,
    ) -> Option<&'a RegisterOverrides>;
}

impl<'a> DerivedRegister<'a> for Register {
    fn name(&self, overrides: Option<&'a RegisterOverrides>) -> String {
        build_ident(
            overrides
                .and_then(|overrides| overrides.name.as_ref())
                .unwrap_or(&self.name),
        )
    }

    fn description(&'a self, overrides: Option<&'a RegisterOverrides>) -> Option<&'a String> {
        overrides
            .and_then(|overrides| overrides.description.as_ref())
            .or(self.description.as_ref())
    }

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

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, RegisterOverrides>>,
    ) -> Option<&'a RegisterOverrides> {
        overrides!(self, overrides)
    }
}
