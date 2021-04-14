use std::fmt::{Display, Formatter, Result};

use svd_parser::Device;

use crate::overrides::DeviceOverrides;
use crate::peripheral::_Peripherals;
use crate::utils::build_ident;

pub(super) struct _Device<'a> {
    pub(super) name: String,
    pub(super) description: Option<&'a String>,
    pub(super) peripherals: _Peripherals<'a>,
}

impl<'a> _Device<'a> {
    pub(super) fn build(device: &'a Device, overrides: Option<&'a DeviceOverrides>) -> _Device<'a> {
        _Device {
            name: device.name(overrides),
            description: device.description(overrides),
            peripherals: _Peripherals::build(device, overrides),
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

trait DerivedDevice<'a> {
    fn name(&self, overrides: Option<&'a DeviceOverrides>) -> String;

    fn description(&'a self, overrides: Option<&'a DeviceOverrides>) -> Option<&'a String>;
}

impl<'a> DerivedDevice<'a> for Device {
    fn name(&self, overrides: Option<&'a DeviceOverrides>) -> String {
        build_ident(
            overrides
                .and_then(|overrides| overrides.name.as_ref())
                .unwrap_or(&self.name),
        )
    }

    fn description(&'a self, overrides: Option<&'a DeviceOverrides>) -> Option<&'a String> {
        overrides
            .and_then(|overrides| overrides.description.as_ref())
            .or(self.description.as_ref())
    }
}
