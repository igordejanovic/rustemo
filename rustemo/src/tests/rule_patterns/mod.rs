use self::types::TypesParser;

#[allow(unused_imports)]
mod rule_patterns;
#[allow(unused_imports)]
mod rule_patterns_actions;

#[test]
fn test_types() {
    let result = RulePatternParser::parse_str("b b b b b");
}
