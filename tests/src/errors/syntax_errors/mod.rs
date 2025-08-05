use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::{local_file, output_cmp};

rustemo_mod!(calc, "/src/errors/syntax_errors");
rustemo_mod!(calc_actions, "/src/errors/syntax_errors");

use self::calc::CalcParser;

#[test]
fn syntax_error_unexpected() {
    let result = CalcParser::new().parse("2 + 3 / 4 + 5");
    output_cmp!(
        local_file!(file!(), "calc_unexpected.err")
            .to_str()
            .unwrap(),
        result.unwrap_err().to_string()
    );
}

#[test]
fn syntax_error_incomplete() {
    let result = CalcParser::new().parse("2 + 3 + 5 +");
    output_cmp!(
        local_file!(file!(), "calc_incomplete.err")
            .to_str()
            .unwrap(),
        result.unwrap_err().to_string()
    );
}
