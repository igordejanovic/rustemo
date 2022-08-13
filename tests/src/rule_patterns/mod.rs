use rustemo::output_cmp;

use crate::rule_patterns::rule_patterns::RulePatternsParser;

#[allow(unused_imports)]
mod rule_patterns;
#[allow(unused_imports)]
mod rule_patterns_actions;

#[test]
fn test_types() {
    let result = RulePatternsParser::parse_str("b b b b b");

    output_cmp!("src/tests/rule_patterns/rule_patterns.ast",
                format!("{result:#?}"));
}
