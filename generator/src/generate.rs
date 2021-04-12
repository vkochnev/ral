use std::fs::{write, DirBuilder};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use crate::cluster::_Cluster;
use crate::device::_Device;
use crate::peripheral::{_Peripheral, _Peripherals};
use crate::register::_Register;

pub(super) fn generate_sources(project_dir: &Path, device: _Device) -> Result<()> {
    let src_dir = project_dir.join("src");
    create_dir(&src_dir)?;

    write(src_dir.join("lib.rs"), device.to_string())?;
    generate_peripherals(&src_dir, device.peripherals)
}

fn generate_peripherals(src_dir: &Path, peripherals: _Peripherals) -> Result<()> {
    let peripherals_dir = src_dir.join("peripherals");
    create_dir(&peripherals_dir)?;

    write(peripherals_dir.join("mod.rs"), peripherals.to_string())?;
    for peripheral in peripherals {
        generate_peripheral(&peripherals_dir, peripheral)?;
    }
    Ok(())
}

fn generate_peripheral(peripherals_dir: &Path, peripheral: _Peripheral) -> Result<()> {
    let peripheral_dir = peripherals_dir.join(peripheral.name.to_string());
    create_dir(&peripheral_dir)?;

    write(peripheral_dir.join("mod.rs"), peripheral.to_string())?;
    for cluster in peripheral.clusters {
        generate_cluster(&peripheral_dir, cluster)?;
    }
    for register in peripheral.registers {
        generate_register(&peripheral_dir, register)?;
    }
    Ok(())
}

fn generate_cluster(parent_dir: &Path, cluster: _Cluster) -> Result<()> {
    let cluster_dir = parent_dir.join(cluster.name.to_string());
    create_dir(&cluster_dir)?;

    write(cluster_dir.join("mod.rs"), cluster.to_string())?;
    for cluster in cluster.clusters {
        generate_cluster(&cluster_dir, cluster)?;
    }
    for register in cluster.registers {
        generate_register(&cluster_dir, register)?;
    }
    Ok(())
}

fn generate_register(parent_dir: &Path, register: _Register) -> Result<()> {
    let register_file = parent_dir.join(format!("{}.rs", register.name.to_string()));
    write(register_file, register.to_string())
}

fn create_dir(dir: &Path) -> Result<()> {
    if dir.exists() {
        if !dir.is_dir() {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("[{}] must be a directory", dir.display()),
            ))?;
        }
        Ok(())
    } else {
        DirBuilder::new().create(dir)
    }
}
