use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(unreach, "/src/glr/regressions/issue_22_panic_get_conflicts");
rustemo_mod!(
    unreach_actions,
    "/src/glr/regressions/issue_22_panic_get_conflicts"
);

use self::unreach::{DefaultBuilder, UnreachParser};

#[test]
fn issue_22_panic_get_conflicts() {
    let result = UnreachParser::new()
        .parse("")
        .unwrap()
        .get_first_tree()
        .unwrap()
        .build(&mut DefaultBuilder::new());
    output_cmp!(
        "src/glr/regressions/issue_22_panic_get_conflicts/result.ast",
        format!("{:#?}", result)
    );
}
