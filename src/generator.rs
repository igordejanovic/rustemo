use std::{fmt::Debug, fs, path::Path};

use super::parser::RustemoParser;

pub(crate) fn generate_parser<F>(grammar_path: F)
where
    F: AsRef<Path> + Debug,
{
    let grammar = RustemoParser::default().parse(
        fs::read_to_string(grammar_path.as_ref())
            .unwrap_or_else(|error| {
                panic!(
                    "Cannot load grammar file {:?}. Error: {:?}",
                    grammar_path, error
                );
            })
            .as_str().into(),
    );

    // TODO: Calculate tables
    //
    // TODO: Generate parser code.

}
