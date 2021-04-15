//! Code generator for [ral](https://docs.rs/ral) crate
//!
//! # How to install
//!
//! ```bash
//! $ cargo install ral-gen
//! ```
//!
//! # How to use
//!
//! ```bash
//! $ cargo ral-gen --svd <svd file location> --overrides <yml file location> --out <target project directory>
//! ```
//! or short form
//! ```bash
//! $ cargo ral-gen -i <svd file location> -e <yml file location> -o <target project directory>
//! ```
//!
//! # Overrides
//!
//! Overrides can be specified in separate `yaml` file of the following structure
//! ```yaml
//! name: <alternate device name> // Must be identifier
//! description: <alternate device description>
//! peripherals:
//!   <peripheral name>: // as can be found in .svd file
//!     name: <alternate peripheral name> // Must be identifier
//!     description: <alternate peripheral description>
//!     features: // List of features for conditional compilation, e.g. packaging names
//!       - <feature1> // to exclude peripheral if it's not available
//!       - <feature2>
//!     clusters:
//!       <cluster name>: // as can be found in .svd file
//!         name: <alternate cluster name> // Must be identifier
//!         description: <alternate cluster description>
//!         features: // List of features for conditional compilation, e.g. packaging names
//!           - <feature1> // to exclude cluster if it's not available
//!           - <feature2>
//!     registers:
//!         <register name>: // as can be found in .svd file
//!           name: <alternate register name> // Must be identifier
//!           description: <alternate register description>
//!           features: // List of features for conditional compilation, e.g. packaging names
//!             - <feature1> // to exclude register if it's not available
//!             - <feature2>
//!           uses: // use expressions required for field type overrides
//!             - crate_name::types::CustomType
//!           fields:
//!             <field name>: // as can be found in .svd file
//!               name: <alternate field name> // Must be identifier
//!               description: <alternate field description>
//!               type: CustomType // Required use expression must be specified in register's uses section
//! ```
//! All fields are optional
//!
//! # What will be generated
//!
//! Module structure described in [ral-macro](https://docs.rs/ral-macro) crate documentation will be generated
//!
//! Also `Cargo.toml` will be generated with content
//! ```toml
//! [package]
//! name = "<device name in lowercase>"
//! description = "<device description if present>"
//! version = "<ral-gen version>"
//! edition = "2018"
//!
//! [dependencies]
//! ral = "<ral-gen version>"
//!
//! [features]
//! feature1 = []
//! feature2 = []
//! ```
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use svd_parser as svd;
use svd_parser::Device;

use crate::overrides::DeviceOverrides;
use crate::package::_Package;

#[macro_use]
mod utils;
mod cluster;
mod device;
mod field;
mod generate;
mod overrides;
mod package;
mod peripheral;
mod register;

/// Generates module structure
pub fn generate(
    svd_file_name: &str,
    overrides_file_name: Option<&str>,
    project_dir: &str,
) -> Result<()> {
    let project_dir = Path::new(project_dir);
    let device = load_device(svd_file_name)?;
    let overrides = load_overrides(overrides_file_name)?;
    generate::generate_package(project_dir, _Package::build(&device, overrides.as_ref()))?;
    Ok(())
}

fn load_device(svd_file_name: &str) -> Result<Device> {
    let file = &mut String::new();
    File::open(svd_file_name)?.read_to_string(file)?;
    svd::parse(file)
}

fn load_overrides(overrides_file_name: Option<&str>) -> Result<Option<DeviceOverrides>> {
    if overrides_file_name.is_none() {
        return Ok(None);
    }
    let file = &mut String::new();
    File::open(overrides_file_name.unwrap())?.read_to_string(file)?;

    let overrides: DeviceOverrides = serde_yaml::from_str(file)?;
    Ok(Some(overrides))
}
