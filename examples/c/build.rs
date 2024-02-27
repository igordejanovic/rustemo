use std::process::exit;

use rustemo_compiler::ParserAlgo;

fn main() {
    let mut settings = rustemo_compiler::Settings::new()
        .table_type(rustemo_compiler::TableType::LALR)
        .parser_algo(ParserAlgo::GLR);
    if std::env::var("CARGO_FEATURE_ARRAYS").is_ok() {
        settings = settings
            .generator_table_type(rustemo_compiler::GeneratorTableType::Arrays);
    }

    if let Err(e) = settings.process_dir() {
        eprintln!("{}", e);
        exit(1);
    }
}
