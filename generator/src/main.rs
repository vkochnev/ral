use std::ffi::{OsStr, OsString};
use std::path::Path;

use clap::{App, Arg, SubCommand};

use ral_gen::generate;

fn is_existing_file_path(path: &OsStr) -> Result<(), OsString> {
    let path = Path::new(path);
    if path.exists() && path.is_file() {
        Ok(())
    } else {
        Err(OsString::from("Path not exists or not a file"))
    }
}

fn is_directory_path(path: &OsStr) -> Result<(), OsString> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(())
    } else if !path.exists()
        && path
            .parent()
            .map(|parent| parent.exists() && parent.is_dir())
            .unwrap_or(false)
    {
        Ok(())
    } else {
        Err(OsString::from(
            "Must be a directory or at least parent directory must exist",
        ))
    }
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let usage = "cargo ral-gen --svd <SVD file> [--overrides <YML file>] --out <Target dir>";
    let app = App::new("cargo-ral-gen")
        .bin_name("cargo")
        .version(version)
        .usage(usage)
        .args(&[
            Arg::with_name("svd")
                .short("i")
                .long("svd")
                .takes_value(true)
                .display_order(0)
                .empty_values(false)
                .global(true)
                .value_name("SVD file")
                .help("Source SVD file location")
                .validator_os(is_existing_file_path),
            Arg::with_name("overrides")
                .short("e")
                .long("overrides")
                .takes_value(true)
                .display_order(1)
                .empty_values(false)
                .global(true)
                .value_name("YML file")
                .help("Overrides yml file location")
                .validator_os(is_existing_file_path),
            Arg::with_name("out")
                .short("o")
                .long("out")
                .takes_value(true)
                .display_order(2)
                .empty_values(false)
                .global(true)
                .value_name("Target dir")
                .help("Target project directory")
                .validator_os(is_directory_path),
        ])
        .subcommand(
            SubCommand::with_name("ral-gen")
                .version(version)
                .usage(usage),
        );
    let args = &app.get_matches();
    if let (_, Some(args)) = args.subcommand() {
        let svd_file = args
            .value_of_os("svd")
            .map(Path::new)
            .expect("SVD file location must be specified");
        let overrides_file = args.value_of_os("overrides").map(Path::new);
        let out_dir = args
            .value_of_os("out")
            .map(Path::new)
            .expect("Output directory location must be specified");
        generate(svd_file, overrides_file, out_dir).expect("Failed to generate sources");
    } else {
        println!("{}", args.usage());
    }
}
