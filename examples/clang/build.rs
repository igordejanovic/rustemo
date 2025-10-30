use std::process::exit;

fn main() {
    let settings = rustemo_compiler::Settings::new()
        .parser_algo(rustemo_compiler::ParserAlgo::GLR)
        .builder_type(rustemo_compiler::BuilderType::Default)
        .fancy_regex(true);
    if let Err(e) = settings.process_dir() {
        eprintln!("{e}");
        exit(1);
    }
}
