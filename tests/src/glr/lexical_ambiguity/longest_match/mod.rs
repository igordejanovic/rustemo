use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(longest_match, "/src/glr/lexical_ambiguity/longest_match");
rustemo_mod!(
    longest_match_actions,
    "/src/glr/lexical_ambiguity/longest_match"
);

use self::longest_match::LongestMatchParser;

#[test]
fn glr_lexical_ambiguity_longest_match() {
    let forest = LongestMatchParser::new().parse("s a 42.42").unwrap();
    assert_eq!(forest.solutions(), 1);

    let mut trees = String::new();
    for tree in &forest {
        trees.push_str(&format!("{tree:#?}\n\n"));
    }
    output_cmp!(
        "src/glr/lexical_ambiguity/longest_match/longest_match.ast",
        format!("{trees}")
    );
}
