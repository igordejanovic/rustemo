use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

use self::zero_or_more_1::ZeroOrMore1Parser;
use self::zero_or_more_1_sep::ZeroOrMore1SepParser;
use self::zero_or_more_2::ZeroOrMore2Parser;

rustemo_mod!(zero_or_more_1, "/src/sugar/zero_or_more");
rustemo_mod!(zero_or_more_1_actions, "/src/sugar/zero_or_more");

rustemo_mod!(zero_or_more_1_sep, "/src/sugar/zero_or_more");
rustemo_mod!(zero_or_more_1_sep_actions, "/src/sugar/zero_or_more");

rustemo_mod!(zero_or_more_2, "/src/sugar/zero_or_more");
rustemo_mod!(zero_or_more_2_actions, "/src/sugar/zero_or_more");

#[test]
fn zero_or_more_1_1() {
    let result = ZeroOrMore1Parser::new().parse("c b a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_2() {
    let result = ZeroOrMore1Parser::new().parse("c  a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_3() {
    let result = ZeroOrMore1Parser::new().parse("c");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_3.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_1_error() {
    let result = ZeroOrMore1Parser::new().parse("a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn zero_or_more_1_2_error() {
    let result = ZeroOrMore1Parser::new().parse("c b b a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_2.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn zero_or_more_1_1_sep() {
    let result = ZeroOrMore1SepParser::new().parse("c b a, a, a, a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1_sep.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_1_sep_error() {
    let result = ZeroOrMore1SepParser::new().parse("c b a, a a, a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1_sep.err",
        result.unwrap_err().to_string()
    );
}

// ANCHOR: zero-or-more-1
#[test]
fn zero_or_more_2_1() {
    let result = ZeroOrMore2Parser::new().parse("c 1 2 3 a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_1.ast",
        format!("{result:#?}")
    );
}
// ANCHOR_END: zero-or-more-1

#[test]
fn zero_or_more_2_2() {
    let result = ZeroOrMore2Parser::new().parse("c a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_2_1_error() {
    let result = ZeroOrMore2Parser::new().parse("c c a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_1.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn zero_or_more_2_2_error() {
    let result = ZeroOrMore2Parser::new().parse("c 1 2 a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_2.err",
        result.unwrap_err().to_string()
    );
}
