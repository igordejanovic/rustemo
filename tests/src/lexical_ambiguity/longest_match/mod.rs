use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(longest_match, "/src/lexical_ambiguity/longest_match");
rustemo_mod!(
    longest_match_actions,
    "/src/lexical_ambiguity/longest_match"
);

use self::longest_match::LongestMatchParser;

#[test]
fn lr_lexical_ambiguity_longest_match() {
    let result = LongestMatchParser::new().parse("s a 42.42").unwrap();

    output_cmp!(
        "src/lexical_ambiguity/longest_match/longest_match.ast",
        format!("{result:?}")
    );
}
