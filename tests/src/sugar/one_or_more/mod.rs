use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

use self::one_or_more_1::OneOrMore1Parser;
use self::one_or_more_1_sep::OneOrMore1SepParser;
use self::one_or_more_2::OneOrMore2Parser;

rustemo_mod!(one_or_more_1, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_1_actions, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_1_sep, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_1_sep_actions, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_2, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_2_actions, "/src/sugar/one_or_more");

#[test]
fn one_or_more_1_1() {
    let result = OneOrMore1Parser::new().parse("c b 1 2 3 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_1_2() {
    let result = OneOrMore1Parser::new().parse("c 1");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_1_1_error() {
    let result = OneOrMore1Parser::new().parse("1 2 3 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_1.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn one_or_more_1_2_error() {
    let result = OneOrMore1Parser::new().parse("c b b 1 2 3 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_2.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn one_or_more_1_3_error() {
    let result = OneOrMore1Parser::new().parse("c b");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_3.err",
        result.unwrap_err().to_string()
    );
}

// ANCHOR: one-or-more-sep
#[test]
fn one_or_more_1_1_sep() {
    let result = OneOrMore1SepParser::new().parse("c b 1, 2, 3, 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_1_sep.ast",
        format!("{result:#?}")
    );
}
// ANCHOR_END: one-or-more-sep
//
#[test]
fn one_or_more_1_1_sep_error() {
    let result = OneOrMore1SepParser::new().parse("c b 1, 2; 3, 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_1_sep.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn one_or_more_2_1() {
    let result = OneOrMore2Parser::new().parse("c 1 a");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_1.ast",
        format!("{result:#?}")
    );
}

// ANCHOR: one-or-more
#[test]
fn one_or_more_2_2() {
    let result = OneOrMore2Parser::new().parse("c 1 2 3 4 a");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_2.ast",
        format!("{result:#?}")
    );
}
// ANCHOR_END: one-or-more

#[test]
fn one_or_more_2_1_error() {
    let result = OneOrMore2Parser::new().parse("c 1 2 3");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_1.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn one_or_more_2_2_error() {
    let result = OneOrMore2Parser::new().parse("c a");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_2.err",
        result.unwrap_err().to_string()
    );
}

#[test]
fn one_or_more_2_3_error() {
    let result = OneOrMore2Parser::new().parse("c 1 2 a 3");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_3.err",
        result.unwrap_err().to_string()
    );
}
