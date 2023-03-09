use rustemo::rustemo_mod;
use rustemo_tools::{local_file, output_cmp};

rustemo_mod!(calculator, "/src/from_file");
rustemo_mod!(calculator_actions, "/src/from_file");

use calculator::CalculatorParser;

#[test]
fn parse_from_file_ok() {
    let mut parser = CalculatorParser::new();
    let result = parser.parse_file(local_file!(file!(), "input1.calc"));
    output_cmp!(
        "src/from_file/parse_from_file_ok.ast",
        format!("{result:#?}")
    )
}

#[test]
fn parse_from_file_err() {
    let mut parser = CalculatorParser::new();
    let result = parser.parse_file(local_file!(file!(), "input2.calc"));
    output_cmp!(
        "src/from_file/parse_from_file_err.err",
        result.unwrap_err().to_string()
    )
}
