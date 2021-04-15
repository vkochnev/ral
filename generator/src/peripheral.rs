use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use svd_parser::{Cluster, Device, Peripheral, Register, RegisterCluster, RegisterProperties};

use crate::cluster::_Cluster;
use crate::overrides::{DeviceOverrides, PeripheralOverrides};
use crate::register::_Register;
use crate::utils::{array_names, build_children, build_ident, merge_defaults};

pub(super) struct _Peripheral<'a> {
    pub(super) name: String,
    description: Option<&'a String>,
    features: Option<&'a Vec<String>>,
    base_address: u64,
    pub(super) clusters: Vec<_Cluster<'a>>,
    pub(super) registers: Vec<_Register<'a>>,
}

impl<'a> _Peripheral<'a> {
    pub(super) fn build(
        peripheral: &'a Peripheral,
        peripherals: &HashMap<String, &'a Peripheral>,
        defaults: RegisterProperties,
        overrides: Option<&'a HashMap<String, PeripheralOverrides>>,
    ) -> _Peripheral<'a> {
        let overrides = peripheral.overrides(overrides);
        let children = peripheral
            .registers(peripherals)
            .expect("There should be no empty peripherals");
        let mut clusters = HashMap::<String, &'a Cluster>::new();
        let mut registers = HashMap::<String, &'a Register>::new();
        Self::collect_clusters_and_registers(&children, &mut clusters, &mut registers);

        let defaults = merge_defaults(
            peripheral.default_register_properties(peripherals),
            defaults,
        );
        let cluster_overrides = overrides.and_then(|overrides| overrides.clusters.as_ref());
        let register_overrides = overrides.and_then(|overrides| overrides.registers.as_ref());
        let (clusters, registers) = build_children(
            children,
            &clusters,
            &registers,
            defaults,
            cluster_overrides,
            register_overrides,
        );
        _Peripheral {
            name: peripheral.name(overrides),
            description: peripheral.description(overrides),
            features: overrides.and_then(|overrides| overrides.features.as_ref()),
            base_address: peripheral.base_address,
            clusters,
            registers,
        }
    }

    fn collect_clusters_and_registers(
        children: &'a Vec<RegisterCluster>,
        clusters: &mut HashMap<String, &'a Cluster>,
        registers: &mut HashMap<String, &'a Register>,
    ) {
        for register_cluster in children {
            match register_cluster {
                RegisterCluster::Cluster(cluster) => {
                    match cluster {
                        Cluster::Single(single) => {
                            clusters.insert(single.name.clone(), cluster);
                        }
                        Cluster::Array(array, dim) => {
                            for name in array_names(&array.name, dim) {
                                clusters.insert(name, cluster);
                            }
                        }
                    }
                    Self::collect_clusters_and_registers(&cluster.children, clusters, registers);
                }
                RegisterCluster::Register(register) => match register {
                    Register::Single(single) => {
                        registers.insert(single.name.clone(), register);
                    }
                    Register::Array(array, dim) => {
                        for name in array_names(&array.name, dim) {
                            registers.insert(name, register);
                        }
                    }
                },
            }
        }
    }
}

impl<'a> Display for _Peripheral<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(description) = self.description {
            write!(
                f,
                "#![doc = \"{description}\"]\n\n",
                description = description
            )?;
        }
        write!(
            f,
            "const BASE_ADDRESS: usize = {base_address:#X};\n\n",
            base_address = self.base_address
        )?;
        write_children!(self, f);
        Ok(())
    }
}

pub(super) struct _Peripherals<'a>(Vec<_Peripheral<'a>>);

impl<'a> _Peripherals<'a> {
    pub(super) fn build(
        device: &'a Device,
        overrides: Option<&'a DeviceOverrides>,
    ) -> _Peripherals<'a> {
        let overrides = overrides.and_then(|overrides| overrides.peripherals.as_ref());
        let mut peripherals = HashMap::<String, &'a Peripheral>::new();
        for peripheral in &device.peripherals {
            peripherals.insert(peripheral.name.clone(), peripheral);
        }
        let defaults = device.default_register_properties;
        let mut collected = Vec::new();
        for peripheral in &device.peripherals {
            collected.push(_Peripheral::build(
                &peripheral,
                &peripherals,
                defaults,
                overrides,
            ));
        }
        _Peripherals(collected)
    }
}

impl<'a> Display for _Peripherals<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for peripheral in &self.0 {
            if let Some(packages_cfg) = features_cfg!(peripheral) {
                write!(f, "{}", packages_cfg)?;
            }
            write!(f, "pub mod {};\n", peripheral.name)?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for _Peripherals<'a> {
    type Item = <Vec<_Peripheral<'a>> as IntoIterator>::Item;
    type IntoIter = <Vec<_Peripheral<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

trait DerivedPeripheral<'a> {
    fn name(&self, overrides: Option<&'a PeripheralOverrides>) -> String;

    fn description(&'a self, overrides: Option<&'a PeripheralOverrides>) -> Option<&'a String>;

    fn default_register_properties(
        &'a self,
        peripherals: &HashMap<String, &'a Peripheral>,
    ) -> RegisterProperties;

    fn registers(
        &'a self,
        peripherals: &HashMap<String, &'a Peripheral>,
    ) -> Option<&'a Vec<RegisterCluster>>;

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, PeripheralOverrides>>,
    ) -> Option<&'a PeripheralOverrides>;
}

impl<'a> DerivedPeripheral<'a> for Peripheral {
    fn name(&self, overrides: Option<&'a PeripheralOverrides>) -> String {
        build_ident(
            overrides
                .and_then(|overrides| overrides.name.as_ref())
                .unwrap_or(&self.name),
        )
    }

    fn description(&'a self, overrides: Option<&'a PeripheralOverrides>) -> Option<&'a String> {
        overrides
            .and_then(|overrides| overrides.description.as_ref())
            .or(self.description.as_ref())
    }

    fn default_register_properties(
        &'a self,
        peripherals: &HashMap<String, &'a Peripheral>,
    ) -> RegisterProperties {
        self.derived_from
            .as_ref()
            .and_then(|name| peripherals.get(name))
            .map(|p| p.default_register_properties(peripherals))
            .map(|derived| merge_defaults(self.default_register_properties, derived))
            .unwrap_or(self.default_register_properties)
    }

    fn registers(
        &'a self,
        peripherals: &HashMap<String, &'a Peripheral>,
    ) -> Option<&'a Vec<RegisterCluster>> {
        let derived = self
            .derived_from
            .as_ref()
            .and_then(|name| peripherals.get(name))
            .and_then(|&p| p.registers(peripherals));
        self.registers.as_ref().or(derived)
    }

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, PeripheralOverrides>>,
    ) -> Option<&'a PeripheralOverrides> {
        overrides!(self, overrides)
    }
}
