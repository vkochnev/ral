//! Code generator for [ral](https://docs.rs/ral) crate
//!
//! # How to install
//!
//! ```bash
//! $ cargo install ral-gen
//! ```
//!
//! # How to use
//! ```bash
//! $ cargo ral-gen --svd <svd file location> --out <target project directory>
//! ```
//! or short form
//! ```bash
//! $ cargo ral-gen -i <svd file location> -o <target project directory>
//! ```
//!
//! # What will be generated
//! Module structure described in [ral-macro](https://docs.rs/ral-macro) crate documentation will be generated
//!
//! # Required `Cargo.toml` content
//! ```toml
//! [package]
//! edition = "2018"
//!
//! [dependencies]
//! ral = "matching ral version"
//! ```
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use svd_parser as svd;
use svd_parser::Device;

use crate::device::_Device;

mod cluster;
mod device;
mod field;
mod generate;
mod peripheral;
mod register;
mod utils;

/// Generates module structure
pub fn generate(svd_file_name: &str, project_dir: &str) -> Result<()> {
    let device = load_device(svd_file_name)?;
    let project_dir = Path::new(project_dir);
    generate::generate_sources(project_dir, _Device::build(&device))?;
    Ok(())
}

fn load_device(svd_file_name: &str) -> Result<Device> {
    let file = &mut String::new();
    File::open(svd_file_name)?.read_to_string(file)?;
    svd::parse(file)
}
