use std::{env, path::PathBuf, process::exit};

use rustemo_compiler::{BuilderType, LexerType, RustemoSettings};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    let settings = rustemo_compiler::with_settings().force(true);

    #[allow(clippy::type_complexity)]
    let tests: &[(
        &str,
        Box<dyn Fn(RustemoSettings) -> RustemoSettings>,
    )] = &[
        ("rule_patterns", Box::new(|s| s)),
        ("sugar", Box::new(|s| s)),
        ("ambiguity", Box::new(|s| s.prefer_shifts(true))),
        ("from_file", Box::new(|s| s)),
        (
            "partial",
            Box::new(|s| s.prefer_shifts(true).partial_parse(true)),
        ),
        // Layout
        ("layout/ast", Box::new(|s| s)),
        (
            "layout/generic_tree",
            Box::new(|s| s.builder_type(BuilderType::Generic)),
        ),
        // Builders
        (
            "builder/generic_tree",
            Box::new(|s| s.builder_type(BuilderType::Generic)),
        ),
        (
            "builder/custom_builder",
            Box::new(|s| s.builder_type(BuilderType::Custom)),
        ),
        (
            "builder/use_context",
            Box::new(|s| {
                // We want actions generated in the source tree.
                s.force(false).actions_in_source_tree()
            }),
        ),
        // Lexer
        (
            "lexer/custom_lexer",
            Box::new(|s| {
                s.lexer_type(LexerType::Custom)
                    .force(false)
                    .actions_in_source_tree()
            }),
        ),
    ];

    for (test, config) in tests {
        let p = format!("src/{test}");
        let dir = out_dir.join(&p);
        if let Err(e) = config(
            settings
                .clone()
                .out_dir_root(dir.clone())
                .out_dir_actions_root(dir),
        )
        .root_dir(root_dir.join(p))
        .process_dir()
        {
            eprintln!("{}", e);
            exit(1);
        }
    }

    // Testing code generation in the source tree
    let settings = rustemo_compiler::with_settings().force(true);
    if let Err(e) = settings
        .in_source_tree()
        .process_grammar(&root_dir.join("src/output_dir/output_dir.rustemo"))
    {
        eprintln!("{}", e);
        exit(1);
    }
    let settings = rustemo_compiler::with_settings().force(true);
    if let Err(e) = settings.actions_in_source_tree().process_grammar(
        &root_dir.join("src/output_dir/output_dir_act.rustemo"),
    ) {
        eprintln!("{}", e);
        exit(1);
    }
}
