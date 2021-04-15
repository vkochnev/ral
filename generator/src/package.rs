use crate::device::_Device;
use crate::overrides::DeviceOverrides;
use indoc::writedoc;
use semver::Version;
use std::fmt::{Display, Formatter, Result};
use svd_parser::Device;

pub(super) struct _Package<'a> {
    pub(super) device: _Device<'a>,
    features: Option<Vec<&'a String>>,
}

impl<'a> _Package<'a> {
    pub(super) fn build(
        device: &'a Device,
        overrides: Option<&'a DeviceOverrides>,
    ) -> _Package<'a> {
        _Package {
            device: _Device::build(device, overrides),
            features: Self::collect_features(overrides),
        }
    }

    fn collect_features(overrides: Option<&'a DeviceOverrides>) -> Option<Vec<&'a String>> {
        overrides
            .and_then(|overrides| overrides.peripherals.as_ref())
            .map(|peripherals| {
                peripherals
                    .values()
                    .flat_map(|peripheral| {
                        let mut collected = Vec::<&'a String>::new();
                        peripheral.features.iter().flat_map(|vec| vec).fold(
                            &mut collected,
                            |collected, value| {
                                collected.push(value);
                                collected
                            },
                        );
                        peripheral
                            .clusters
                            .iter()
                            .flat_map(|map| map.values())
                            .flat_map(|overrides| overrides.features.iter())
                            .flat_map(|vec| vec)
                            .fold(&mut collected, |collected, value| {
                                collected.push(value);
                                collected
                            });
                        peripheral
                            .registers
                            .iter()
                            .flat_map(|map| map.values())
                            .flat_map(|overrides| overrides.features.iter())
                            .flat_map(|vec| vec)
                            .fold(&mut collected, |collected, value| {
                                collected.push(value);
                                collected
                            });
                        collected.sort();
                        collected.dedup();
                        collected
                    })
                    .collect::<Vec<&'a String>>()
            })
            .and_then(|features| {
                if features.is_empty() {
                    None
                } else {
                    Some(features)
                }
            })
    }
}

impl<'a> Display for _Package<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writedoc!(
            f,
            "
            [package]
            name = \"{name}\"
            ",
            name = &self.device.name
        )?;
        if let Some(description) = self.device.description {
            write!(
                f,
                "description = \"{description}\"\n",
                description = description
            )?;
        }
        let version = env!("CARGO_PKG_VERSION");
        let mut ral_version = Version::parse(version).map_err(|_| std::fmt::Error::default())?;
        ral_version.patch = 0;
        writedoc!(
            f,
            "
            version = \"{version}\"
            edition = \"2018\"
            
            [dependencies]
            ral = \"{ral_version}\"
            ",
            version = version,
            ral_version = ral_version
        )?;
        if let Some(features) = self.features.as_ref() {
            write!(f, "\n[features]\n")?;
            for feature in features {
                write!(f, "{feature} = []\n", feature = feature)?;
            }
        }
        Ok(())
    }
}
