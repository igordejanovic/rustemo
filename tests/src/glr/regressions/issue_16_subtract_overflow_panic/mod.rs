use rustemo::{rustemo_mod, Parser};

rustemo_mod!(
    inline,
    "/src/glr/regressions/issue_16_subtract_overflow_panic"
);
rustemo_mod!(
    inline_actions,
    "/src/glr/regressions/issue_16_subtract_overflow_panic"
);

use self::inline::{DefaultBuilder, InlineParser};

#[test]
fn subtract_overflow() {
    InlineParser::new()
        .parse("*ld 2")
        .unwrap()
        .get_first_tree()
        .unwrap()
        .build(&mut DefaultBuilder::new());
}
