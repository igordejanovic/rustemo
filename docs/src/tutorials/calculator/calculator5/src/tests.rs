// Use the generated parser
use crate::calculator::CalculatorParser;
use rustemo_tools::output_cmp;

#[test]
fn calculator5_1() {
    let result = CalculatorParser::parse("2 + 3 * 7 / 2.3");
    assert_eq!(result.unwrap(), 2.0 + 3.0 * 7.0 / 2.3);
}

#[test]
fn calculator5_1_error() {
    let result = CalculatorParser::parse("2 + 3 * 7 ^ 2.3");
    output_cmp!("src/calculator5_1.err", result.unwrap_err().to_string());
}
