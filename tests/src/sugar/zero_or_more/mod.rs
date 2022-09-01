use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;

use self::zero_or_more_1::ZeroOrMore1Parser;
use self::zero_or_more_2::ZeroOrMore2Parser;

pub type Input = str;

rustemo_mod!(zero_or_more_1, "/src/sugar/zero_or_more");
rustemo_mod!(zero_or_more_1_actions, "/src/sugar/zero_or_more");

rustemo_mod!(zero_or_more_2, "/src/sugar/zero_or_more");
rustemo_mod!(zero_or_more_2_actions, "/src/sugar/zero_or_more");

#[test]
fn zero_or_more_1_1() {
    let result = ZeroOrMore1Parser::parse("c b a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_2() {
    let result = ZeroOrMore1Parser::parse("c  a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_3() {
    let result = ZeroOrMore1Parser::parse("c");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_3.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_1_error() {
    let result = ZeroOrMore1Parser::parse("a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_1_2_error() {
    let result = ZeroOrMore1Parser::parse("c b b a a a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_1_2_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_2_1() {
    let result = ZeroOrMore2Parser::parse("c 1 2 3 a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_2_2() {
    let result = ZeroOrMore2Parser::parse("c a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_2_1_error() {
    let result = ZeroOrMore2Parser::parse("c c a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_2_2_error() {
    let result = ZeroOrMore2Parser::parse("c 1 2 a a");
    output_cmp!(
        "src/sugar/zero_or_more/zero_or_more_2_2_error.ast",
        format!("{result:#?}")
    );
}
