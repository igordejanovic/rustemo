use std::{env, path::PathBuf, process::exit};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    if let Err(e) = rustemo::with_settings()
        .out_dir(&out_dir)
        .out_dir_actions(&out_dir)
        // TODO: Temporary exclude ambiguities tests until issues solved
        .exclude(vec!["ambiguity".into()])
        .force(true)
        .process_dir(&root_dir)
    {
        eprintln!("{}", e);
        exit(1);
    }
}
