use std::{env, path::PathBuf, process::exit};

use rustemo::api::RustemoSettings;

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    let settings = rustemo::with_settings().force(true);

    let tests: &[(&str, Box<dyn Fn(RustemoSettings) -> RustemoSettings>)] = &[
        ("rule_patterns", Box::new(|s| s)),
        ("sugar", Box::new(|s| s)),
        ("ambiguity", Box::new(|s| s.prefer_shifts(true))),
        (
            "partial",
            Box::new(|s| s.prefer_shifts(true).partial_parse(true)),
        ),
        (
            "pass_context",
            Box::new(|s| {
                s.pass_context(true).force(false).out_dir_actions(None)
            }),
        ),
        ("layout", Box::new(|s| s)),
    ];

    for (test, config) in tests {
        let p = format!("src/{test}");
        let dir = out_dir.join(&p);
        if let Err(e) = config(
            settings
                .clone()
                .out_dir(Some(dir.clone()))
                .out_dir_actions(Some(dir)),
        )
        .process_dir(&root_dir.join(p))
        {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
