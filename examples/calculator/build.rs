use std::{process::exit, path::PathBuf};

fn main() {
    let root_dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "src"].iter().collect();
    if let Err(e) = rustemo::generate_parsers(root_dir) {
        eprintln!("{}", e);
        exit(1);
    }
}
