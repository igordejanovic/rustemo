use std::{env, path::PathBuf, process::exit};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    if let Err(e) = rustemo_compiler::with_settings()
        .out_dir(Some(out_dir))
        .process_dir(&root_dir)
    {
        eprintln!("{}", e);
        exit(1);
    }
}
