use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(priorities, "/src/lexical_ambiguity/priorities");
rustemo_mod!(priorities_actions, "/src/lexical_ambiguity/priorities");
rustemo_mod!(priorities_same, "/src/lexical_ambiguity/priorities");
rustemo_mod!(priorities_same_actions, "/src/lexical_ambiguity/priorities");

use self::priorities::PrioritiesParser;
use self::priorities_same::PrioritiesSameParser;

#[test]
fn lr_lexical_ambiguity_priorities() {
    let result = PrioritiesParser::new().parse("a firstone").unwrap();

    output_cmp!(
        "src/lexical_ambiguity/priorities/priorities.ast",
        format!("{result:?}")
    );
}

#[test]
fn lr_lexical_ambiguity_priorities_same() {
    let result = PrioritiesSameParser::new().parse("a firstone").unwrap();

    output_cmp!(
        "src/lexical_ambiguity/priorities/priorities_same.ast",
        format!("{result:?}")
    );
}
