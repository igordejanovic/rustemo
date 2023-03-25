// Use the generated parser
use crate::calculator::CalculatorParser;
use rustemo_compiler::output_cmp;

#[test]
fn calculator4_1() {
    let result = CalculatorParser::parse("2 + 3 * 7 / 2.3");
    output_cmp!("src/calculator4_1.ast", format!("{result:#?}"));
}

#[test]
fn calculator4_1_error() {
    let result = CalculatorParser::parse("2 + 3 * 7 ^ 2.3");
    output_cmp!("src/calculator4_1.err", result.unwrap_err().to_string());
}
