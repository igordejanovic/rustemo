use std::{path::PathBuf, process::exit, env};

use rustemo::settings::Settings;

fn main() {
    let root_dir: PathBuf =
        [env!("CARGO_MANIFEST_DIR"), "src"].iter().collect();

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    if let Err(e) = rustemo::generate_parsers(
        &root_dir,
        Some(&out_dir.join("src")), None,
        &Settings::default(),
    ) {
        eprintln!("{}", e);
        exit(1);
    }
}
