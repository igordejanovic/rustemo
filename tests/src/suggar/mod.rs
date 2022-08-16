use rustemo_rt::rustemo_parser;

use self::zero_or_more::ZeroOrMoreParser;

rustemo_parser!(zero_or_more, "/src/suggar");
#[allow(unused_imports)]
mod zero_or_more_actions;


#[test]
fn test_zero_or_more() {
    let result = ZeroOrMoreParser::parse_str("a a a a");
}
