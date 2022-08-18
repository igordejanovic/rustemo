use rustemo::output_cmp;
use rustemo_rt::rustemo_parser;

use self::zero_or_more_1::ZeroOrMore1Parser;

rustemo_parser!(zero_or_more_1, "/src/sugar/zero_or_more");
rustemo_parser!(zero_or_more_1_actions, "/src/sugar/zero_or_more");

#[test]
fn zero_or_more_1_1() {
    let result = ZeroOrMore1Parser::parse_str("c b a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_2() {
    let result = ZeroOrMore1Parser::parse_str("c  a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_3() {
    let result = ZeroOrMore1Parser::parse_str("c");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_3.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_1_error() {
    let result = ZeroOrMore1Parser::parse_str("a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_2_error() {
    let result = ZeroOrMore1Parser::parse_str("c b b a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_2_error.ast",
        format!("{result:#?}")
    );
}
