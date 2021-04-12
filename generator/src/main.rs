use std::path::PathBuf;

use structopt::StructOpt;

use ral_gen::generate;

#[derive(StructOpt)]
struct CliArgs {
    #[structopt(short = "i", long = "svd", parse(from_os_str))]
    svd_file: PathBuf,
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    out_dir: PathBuf,
}

fn main() {
    let args: CliArgs = CliArgs::from_args();
    let svd_file_name = args
        .svd_file
        .to_str()
        .expect("SVD file location must be specified");
    let out_dir = args
        .out_dir
        .to_str()
        .expect("Output directory location must be specified");
    generate(svd_file_name, out_dir).expect("Failed to generate sources");
}
