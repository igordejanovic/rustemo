use std::{env, path::PathBuf, process::exit};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    if let Err(e) = rustemo::with_settings()
        .out_dir(&out_dir)
        .out_dir_actions(&out_dir)
        .exclude(vec!["ambiguity".into()])
        .force(true)
        .process_dir(&root_dir)
    {
        eprintln!("{}", e);
        exit(1);
    }

    // Special handling of ambiguous grammars by using prefer_shifts strategy.
    let p = "src/ambiguity";
    let dir = out_dir.join(p);
    if let Err(e) = rustemo::with_settings()
        .out_dir(&dir)
        .out_dir_actions(&dir)
        .force(true)
        .prefer_shifts(true)
        .process_dir(&root_dir.join(p))
    {
        eprintln!("{}", e);
        exit(1);
    }

    // Partial parse
    let p = "src/partial";
    let dir = out_dir.join(p);
    if let Err(e) = rustemo::with_settings()
        .out_dir(&dir)
        .out_dir_actions(&dir)
        .force(true)
        .prefer_shifts(true)
        .partial_parse(true)
        .process_dir(&root_dir.join(p))
    {
        eprintln!("{}", e);
        exit(1);
    }
}
