use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;

use self::optional_1::Optional1Parser;
use self::optional_2::Optional2Parser;

rustemo_mod!(optional_1, "/src/sugar/optional");
rustemo_mod!(optional_1_actions, "/src/sugar/optional");

rustemo_mod!(optional_2, "/src/sugar/optional");
rustemo_mod!(optional_2_actions, "/src/sugar/optional");

#[test]
fn optional_1_1() {
    let result = Optional1Parser::parse("c b 1");
    output_cmp!(
        "src/sugar/optional/optional_1_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_1_2() {
    let result = Optional1Parser::parse("c b");
    output_cmp!(
        "src/sugar/optional/optional_1_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_1_3() {
    let result = Optional1Parser::parse("b 1");
    output_cmp!(
        "src/sugar/optional/optional_1_3.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_1_1_error() {
    let result = Optional1Parser::parse("c 1");
    output_cmp!(
        "src/sugar/optional/optional_1_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_1_2_error() {
    let result = Optional1Parser::parse("c b 1 2");
    output_cmp!(
        "src/sugar/optional/optional_1_2_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_2_1() {
    let result = Optional1Parser::parse("c 1");
    output_cmp!(
        "src/sugar/optional/optional_2_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_2_2() {
    let result = Optional1Parser::parse("c 1 a");
    output_cmp!(
        "src/sugar/optional/optional_2_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_2_3() {
    let result = Optional1Parser::parse("c a");
    output_cmp!(
        "src/sugar/optional/optional_2_3.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_2_1_error() {
    let result = Optional1Parser::parse("c 1 2");
    output_cmp!(
        "src/sugar/optional/optional_2_1_error.ast",
        format!("{result:#?}")
    );
}

#[test]
fn optional_2_2_error() {
    let result = Optional1Parser::parse("c a a");
    output_cmp!(
        "src/sugar/optional/optional_2_2_error.ast",
        format!("{result:#?}")
    );
}
