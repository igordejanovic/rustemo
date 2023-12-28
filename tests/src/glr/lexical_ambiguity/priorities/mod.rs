use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(priorities, "/src/glr/lexical_ambiguity/priorities");
rustemo_mod!(priorities_actions, "/src/glr/lexical_ambiguity/priorities");
rustemo_mod!(priorities_same, "/src/glr/lexical_ambiguity/priorities");
rustemo_mod!(
    priorities_same_actions,
    "/src/glr/lexical_ambiguity/priorities"
);

use self::priorities::PrioritiesParser;
use self::priorities_same::PrioritiesSameParser;

#[test]
fn glr_lexical_ambiguity_priorities() {
    let forest = PrioritiesParser::new().parse("a firstone").unwrap();
    assert_eq!(forest.solutions(), 1);

    let mut trees = String::new();
    for tree in &forest {
        trees.push_str(&format!("{tree:#?}\n\n"));
    }
    output_cmp!(
        "src/glr/lexical_ambiguity/priorities/priorities.ast",
        format!("{trees}")
    );
}

#[test]
fn glr_lexical_ambiguity_priorities_same() {
    let forest = PrioritiesSameParser::new().parse("a firstone").unwrap();
    assert_eq!(forest.solutions(), 1);

    let mut trees = String::new();
    for tree in &forest {
        trees.push_str(&format!("{tree:#?}\n\n"));
    }
    output_cmp!(
        "src/glr/lexical_ambiguity/priorities/priorities_same.ast",
        format!("{trees}")
    );
}
