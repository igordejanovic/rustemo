use self::zero_or_more::ZeroOrMoreParser;

#[allow(unused_imports)]
mod zero_or_more;
#[allow(unused_imports)]
mod zero_or_more_actions;


#[test]
fn test_zero_or_more() {
    let result = ZeroOrMoreParser::parse_str("a a a a");
}
