use std::{env, path::PathBuf, process::exit};

use rustemo_compiler::{BuilderType, LexerType, ParserAlgo, Settings};

fn main() {
    fn default_settings() -> Settings {
        let mut settings = rustemo_compiler::Settings::new().force(true);
        if std::env::var("CARGO_FEATURE_ARRAYS").is_ok() {
            settings = settings.generator_table_type(rustemo_compiler::GeneratorTableType::Arrays);
        }
        settings
    }
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    #[allow(clippy::type_complexity)]
    let tests: &[(&str, Box<dyn Fn(Settings) -> Settings>)] = &[
        ("rule_patterns", Box::new(|s| s)),
        ("sugar", Box::new(|s| s)),
        ("unicode", Box::new(|s| s)),
        ("fancy_regex", Box::new(|s| s.fancy_regex(true))),
        ("errors/syntax_errors", Box::new(|s| s)),
        ("ambiguity", Box::new(|s| s.prefer_shifts(true))),
        // LR lexical ambiguities
        ("lexical_ambiguity/priorities", Box::new(|s| s)),
        ("lexical_ambiguity/most_specific", Box::new(|s| s)),
        (
            "lexical_ambiguity/most_specific_off",
            Box::new(|s| s.lexical_disamb_most_specific(false)),
        ),
        (
            "lexical_ambiguity/longest_match",
            Box::new(|s| s.lexical_disamb_most_specific(false)),
        ),
        (
            "lexical_ambiguity/grammar_order",
            Box::new(|s| {
                s.lexical_disamb_most_specific(false)
                    .lexical_disamb_longest_match(false)
            }),
        ),
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
        ("builder/loc_info", Box::new(|s| s.builder_loc_info(true))),
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
        ("special/pager_g1", Box::new(|s| s)),
        ("special/lalrpop768", Box::new(|s| s)),
        // GLR
        ("glr/errors", Box::new(|s| s.parser_algo(ParserAlgo::GLR))),
        ("glr/forest", Box::new(|s| s.parser_algo(ParserAlgo::GLR))),
        // GLR builders
        (
            "glr/build/basic",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/build/loc_info",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR).builder_loc_info(true)),
        ),
        // GLR lexical ambiguities
        (
            "glr/lexical_ambiguity/priorities",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/lexical_ambiguity/most_specific",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/lexical_ambiguity/most_specific_off",
            Box::new(|s| {
                s.parser_algo(ParserAlgo::GLR)
                    .lexical_disamb_most_specific(false)
                    .lexical_disamb_longest_match(false)
            }),
        ),
        (
            "glr/lexical_ambiguity/longest_match",
            Box::new(|s| {
                s.parser_algo(ParserAlgo::GLR)
                    .lexical_disamb_most_specific(false)
            }),
        ),
        (
            "glr/lexical_ambiguity/longest_match_off",
            Box::new(|s| {
                s.parser_algo(ParserAlgo::GLR)
                    .lexical_disamb_most_specific(false)
                    .lexical_disamb_longest_match(false)
            }),
        ),
        (
            "glr/lexical_ambiguity/grammar_order",
            Box::new(|s| {
                s.parser_algo(ParserAlgo::GLR)
                    .lexical_disamb_most_specific(false)
                    .lexical_disamb_longest_match(false)
                    .lexical_disamb_grammar_order(true)
            }),
        ),
        (
            "glr/lexical_ambiguity/grammar_order_off",
            Box::new(|s| {
                s.parser_algo(ParserAlgo::GLR)
                    .lexical_disamb_most_specific(false)
                    .lexical_disamb_longest_match(false)
                    .lexical_disamb_grammar_order(false)
            }),
        ),
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
        (
            "glr/special/highly_ambiguous",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/reduce_enough_empty",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/reduce_enough_many_empty",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/bounded_ambiguity",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/bounded_direct_ambiguity",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/unbounded_ambiguity",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/farshi_g7",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/special/farshi_g8",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
        (
            "glr/regressions/issue_16_subtract_overflow_panic",
            Box::new(|s| s.parser_algo(ParserAlgo::GLR)),
        ),
    ];

    for (test, config) in tests {
        let p = format!("src/{test}");
        let dir = out_dir.join(&p);
        if let Err(e) = config(
            default_settings()
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
    if let Err(e) = default_settings()
        .in_source_tree()
        .process_grammar(&root_dir.join("src/output_dir/output_dir.rustemo"))
    {
        eprintln!("{}", e);
        exit(1);
    }
    if let Err(e) = default_settings()
        .actions_in_source_tree()
        .process_grammar(&root_dir.join("src/output_dir/output_dir_act.rustemo"))
    {
        eprintln!("{}", e);
        exit(1);
    }
}
