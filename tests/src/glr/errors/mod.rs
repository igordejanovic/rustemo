use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(calc, "/src/glr/errors");
rustemo_mod!(calc_actions, "/src/glr/errors");
use self::calc::CalcParser;

#[test]
fn glr_syntax_error_unexpected() {
    let input = "1 + 4 * 9 ! 3 * 2 + 7";
    let result = CalcParser::new().parse(input);
    output_cmp!(
        "src/glr/errors/calc_unexpected.err",
        format!("{:#?}", result.unwrap_err())
    );
}

#[test]
fn glr_syntax_error_missing() {
    let input = "1 + 4 * 9 3 * 2 + 7";
    let result = CalcParser::new().parse(input);
    output_cmp!(
        "src/glr/errors/calc_missing.err",
        format!("{:#?}", result.unwrap_err())
    );
}

#[test]
fn glr_syntax_error_incomplete() {
    let input = "1 + 4 * 9 + 3 * 2 +";
    let result = CalcParser::new().parse(input);
    output_cmp!(
        "src/glr/errors/calc_incomplete.err",
        format!("{:#?}", result.unwrap_err())
    );
}
