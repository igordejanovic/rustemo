// Use the generated parser
use crate::calculator::CalculatorParser;
use rustemo_tools::output_cmp;

#[test]
fn calculator1_1() {
    let result = CalculatorParser::parse("2 + 3");
    output_cmp!("src/calculator1_1.ast", format!("{result:#?}"));
}

#[test]
fn calculator1_1_error() {
    let result = CalculatorParser::parse("2 3");
    output_cmp!("src/calculator1_1.err", result.unwrap_err().to_string());
}
