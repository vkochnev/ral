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
