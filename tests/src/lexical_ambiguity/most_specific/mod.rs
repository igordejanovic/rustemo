use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(most_specific, "/src/lexical_ambiguity/most_specific");
rustemo_mod!(
    most_specific_actions,
    "/src/lexical_ambiguity/most_specific"
);

use self::most_specific::MostSpecificParser;

#[test]
fn lr_lexical_ambiguity_most_specific() {
    let result = MostSpecificParser::new().parse("s a 42.42").unwrap();

    output_cmp!(
        "src/lexical_ambiguity/most_specific/most_specific.ast",
        format!("{result:?}")
    );
}
