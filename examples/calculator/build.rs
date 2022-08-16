use std::{path::PathBuf, process::exit, env};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    if let Err(e) = rustemo::with_settings()
        .out_dir(out_dir)
        .process_dir(&root_dir)
    {
        eprintln!("{}", e);
        exit(1);
    }
}
