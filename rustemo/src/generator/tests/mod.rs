use std::path::PathBuf;

use super::generate_parser;

#[test]
fn test_calc_generate() {
    let mut grammar_file = PathBuf::from(file!());
    grammar_file.pop();
    grammar_file.push("calc");
    grammar_file.push("calc.rustemo");

    generate_parser(grammar_file).unwrap();

}

#[test]
fn test_rustemo_generate() {
    let mut grammar_file = PathBuf::from(file!());
    grammar_file.pop();
    grammar_file.push("rustemo");
    grammar_file.push("rustemo.rustemo");

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
