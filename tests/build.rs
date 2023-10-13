use std::{env, path::PathBuf, process::exit};

use rustemo_compiler::{BuilderType, LexerType, ParserAlgo, Settings};

fn main() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    let settings = rustemo_compiler::Settings::new().force(true);

    #[allow(clippy::type_complexity)]
    let tests: &[(&str, Box<dyn Fn(Settings) -> Settings>)] = &[
        ("rule_patterns", Box::new(|s| s)),
        ("sugar", Box::new(|s| s)),
        ("unicode", Box::new(|s| s)),
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
                    .input_type("[u8]".into())
                    .force(false)
                    .actions_in_source_tree()
            }),
        ),
        // Special
        ("special/lalr_reduce_reduce_conflict", Box::new(|s| s)),
        ("special/nondeterministic_palindromes", Box::new(|s| s)),
        // GLR
        ("glr/forest", Box::new(|s| s.parser_algo(ParserAlgo::GLR))),
        ("glr/build", Box::new(|s| s.parser_algo(ParserAlgo::GLR))),
        (
            "glr/evaluate",
            Box::new(|s| {
                s.force(false)
                    .parser_algo(ParserAlgo::GLR)
                    .actions_in_source_tree()
            }),
        ),
        // GLR special
        (
            "glr/special/knuth_lr1",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/nondeterministic_palindromes",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/cyclic_1",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR).print_table(true)),
        ),
        (
            "glr/special/cyclic_2",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR).print_table(true)),
        ),
        (
            "glr/special/right_nullable",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
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
    let settings = rustemo_compiler::Settings::new().force(true);
    if let Err(e) = settings
        .in_source_tree()
        .process_grammar(&root_dir.join("src/output_dir/output_dir.rustemo"))
    {
        eprintln!("{}", e);
        exit(1);
    }
    let settings = rustemo_compiler::Settings::new().force(true);
    if let Err(e) = settings.actions_in_source_tree().process_grammar(
        &root_dir.join("src/output_dir/output_dir_act.rustemo"),
    ) {
        eprintln!("{}", e);
        exit(1);
    }
}
