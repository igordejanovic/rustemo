use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(most_specific, "/src/lexical_ambiguity/most_specific_off");
rustemo_mod!(
    most_specific_actions,
    "/src/lexical_ambiguity/most_specific_off"
);

use self::most_specific::MostSpecificParser;

#[test]
fn lr_lexical_ambiguity_most_specific_off() {
    let result = MostSpecificParser::new().parse("s a 42.42").unwrap();

    output_cmp!(
        "src/lexical_ambiguity/most_specific_off/most_specific.ast",
        format!("{result:?}")
    );
}
