use std::{path::PathBuf, process::exit};

mod calculator;
mod calculator_actions;

fn main() {
    let grammar_file: PathBuf = [env!("CARGO_MANIFEST_DIR"),
                                 "src", "calculator.rustemo"].iter().collect();
    if let Err(e) = rustemo::generate_parser(grammar_file) {
        eprintln!("{}", e);
        exit(1);
    }
    // println!(
    //     "{:?}",
    //     calculator::CalculatorParser::default().parse_default("2 + 3".into())
    // );
}

#[test]
fn test_parse_1() {
    let mut parser = calculator::CalculatorParser::default();
    let ast = parser.parse_default("2 + 3 * 7 + 6 * 3".into());
    assert_eq!(ast, 41);
}

#[test]
fn test_parse_2() {
    let mut parser = calculator::CalculatorParser::default();
    let ast = parser.parse_default("2 + ( 3  * 7 ) + 2 * 4".into());
    assert_eq!(ast, 31);
}
