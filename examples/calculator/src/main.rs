use std::{path::PathBuf, process::exit};

mod calculator1;
mod calculator1_actions;
mod calculator2;
mod calculator2_actions;

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
fn test_calculator1_1() {
    let mut parser = calculator1::Calculator1Parser::default();
    let ast = parser.parse_default("2 + 3 * 7 + 6 * 3".into());
    assert_eq!(ast, 41);
}

#[test]
fn test_calculator1_2() {
    let mut parser = calculator1::Calculator1Parser::default();
    let ast = parser.parse_default("2 + ( 3  * 7 ) + 2 * 4".into());
    assert_eq!(ast, 31);
}

#[test]
fn test_calculator2_1() {
    let mut parser = calculator2::Calculator2Parser::default();
    let ast = parser.parse_default("7 + 56.4 / 3 + 5 / 2 * (7 - 1)".into());
    assert_eq!(ast, 40.800003f32);
}
