use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;

use self::one_or_more_1::OneOrMore1Parser;
use self::one_or_more_2::OneOrMore2Parser;

rustemo_mod!(one_or_more_1, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_1_actions, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_2, "/src/sugar/one_or_more");
rustemo_mod!(one_or_more_2_actions, "/src/sugar/one_or_more");

#[test]
fn one_or_more_1_1() {
    let result = OneOrMore1Parser::parse_str("c b 1 2 3 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_1_2() {
    let result = OneOrMore1Parser::parse_str("c 1");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_1_1_error() {
    let result = OneOrMore1Parser::parse_str("1 2 3 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_1_2_error() {
    let result = OneOrMore1Parser::parse_str("c b b 1 2 3 4");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_2_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_1_3_error() {
    let result = OneOrMore1Parser::parse_str("c b");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_1_3_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_2_1() {
    let result = OneOrMore2Parser::parse_str("c 1 a");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_2_2() {
    let result = OneOrMore2Parser::parse_str("c 1 2 3 4 a");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_2_1_error() {
    let result = OneOrMore2Parser::parse_str("c 1 2 3");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_2_2_error() {
    let result = OneOrMore2Parser::parse_str("c a");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_2_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more_2_3_error() {
    let result = OneOrMore2Parser::parse_str("c 1 2 a 3");
    output_cmp!(
        "src/sugar/one_or_more/one_or_more_2_3_error.ast",
        format!("{result:#?}")
    );
}
