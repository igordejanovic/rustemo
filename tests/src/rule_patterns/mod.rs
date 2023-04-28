use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

rustemo_mod!(zero_or_more_1, "/src/rule_patterns");
rustemo_mod!(zero_or_more_1_actions, "/src/rule_patterns");
use self::zero_or_more_1::ZeroOrMore1Parser;

rustemo_mod!(zero_or_more_2, "/src/rule_patterns");
rustemo_mod!(zero_or_more_2_actions, "/src/rule_patterns");
use self::zero_or_more_2::ZeroOrMore2Parser;

rustemo_mod!(one_or_more, "/src/rule_patterns");
rustemo_mod!(one_or_more_actions, "/src/rule_patterns");
use self::one_or_more::OneOrMoreParser;

rustemo_mod!(optional, "/src/rule_patterns");
rustemo_mod!(optional_actions, "/src/rule_patterns");
use self::optional::OptionalParser;

#[test]
fn zero_or_more_1() {
    let result = ZeroOrMore1Parser::new().parse("1 2 3");

    output_cmp!(
        "src/rule_patterns/zero_or_more_1.ast",
        format!("{result:#?}")
    );
}

#[test]
fn zero_or_more_2() {
    let result = ZeroOrMore2Parser::new().parse("1 2 3");

    output_cmp!(
        "src/rule_patterns/zero_or_more_2.ast",
        format!("{result:#?}")
    );
}

#[test]
fn one_or_more() {
    let result = OneOrMoreParser::new().parse("1 2 3");

    output_cmp!("src/rule_patterns/one_or_more.ast", format!("{result:#?}"));
}

#[test]
fn optional() {
    let result = OptionalParser::new().parse("1");

    output_cmp!("src/rule_patterns/optional.ast", format!("{result:#?}"));
}
