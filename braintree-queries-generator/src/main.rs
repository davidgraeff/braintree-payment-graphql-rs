extern crate failure;
#[cfg(feature = "rustfmt")]
extern crate rustfmt_nightly;
extern crate syn;
use std::path::PathBuf;

mod generate;
use generate::generate;

fn main() -> Result<(), failure::Error> {
    let query_path: PathBuf = "queries/".parse().expect("Query path error");
    let output_directory: PathBuf = "src/queries".parse().expect("output directory 'src/queries' error");
    std::fs::create_dir_all(&output_directory)?;

    println!("Generating query sources for {}", query_path.to_str().unwrap());
    println!(
        "Formatting is {}",
        match cfg!(feature = "rustfmt") {
            true => "Enabled",
            false => "Disabled",
        }
    );

    generate(query_path, output_directory)
}
