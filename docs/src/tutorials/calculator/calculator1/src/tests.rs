// Use the generated parser
use crate::calculator::CalculatorParser;
use rustemo::Parser;
use rustemo_compiler::output_cmp;

#[test]
fn calculator1_1() {
    let result = CalculatorParser::new().parse("2 + 3");
    output_cmp!("src/calculator1_1.ast", format!("{result:#?}"));
}

#[test]
fn calculator1_1_error() {
    let result = CalculatorParser::new().parse("2 3");
    output_cmp!("src/calculator1_1.err", result.unwrap_err().to_string());
}
