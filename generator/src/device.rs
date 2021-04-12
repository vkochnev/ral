use std::fmt::{Display, Formatter, Result};

use svd_parser::Device;

use crate::peripheral::_Peripherals;

pub(super) struct _Device<'a> {
    description: &'a Option<String>,
    pub(super) peripherals: _Peripherals<'a>,
}

impl<'a> _Device<'a> {
    pub(super) fn build(device: &'a Device) -> _Device<'a> {
        _Device {
            description: &device.description,
            peripherals: _Peripherals::build(device),
        }
    }
}

impl<'a> Display for _Device<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(description) = self.description {
            write!(
                f,
                "#![doc = \"{description}\"]\n\n",
                description = description
            )?;
        }
        write!(f, "pub mod peripherals;")
    }
}
