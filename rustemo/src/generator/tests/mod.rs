use std::path::PathBuf;

use super::generate_parser;

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn test_calc_generate() {
    let mut grammar_file = test_dir();
    grammar_file.push("src/generator/tests/calc/calc.rustemo");

    if let Err(e) = generate_parser(grammar_file) {
        eprintln!("{}", e);
        panic!()
    }

}

#[test]
fn test_rustemo_generate() {
    let mut grammar_file = test_dir();
    grammar_file.push("src/generator/tests/rustemo/rustemo.rustemo");

    generate_parser(grammar_file).unwrap();

}

#[test]
fn test_rustemo_bootstrap() {
    let mut grammar_file = PathBuf::from(file!());
    grammar_file.pop();
    grammar_file.push("rustemo");
    grammar_file.push("rustemo.rustemo");

    generate_parser(grammar_file).unwrap();

}
