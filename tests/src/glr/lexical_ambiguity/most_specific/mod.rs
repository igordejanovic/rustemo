use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(most_specific, "/src/glr/lexical_ambiguity/most_specific");
rustemo_mod!(
    most_specific_actions,
    "/src/glr/lexical_ambiguity/most_specific"
);

use self::most_specific::MostSpecificParser;

#[test]
fn glr_lexical_ambiguity_most_specific() {
    let forest = MostSpecificParser::new().parse("s a 42.42").unwrap();
    assert_eq!(forest.solutions(), 1);

    let mut trees = String::new();
    for tree in &forest {
        trees.push_str(&format!("{tree:#?}\n\n"));
    }
    output_cmp!(
        "src/glr/lexical_ambiguity/most_specific/most_specific.ast",
        format!("{trees}")
    );
}
