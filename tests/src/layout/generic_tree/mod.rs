use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

use self::layout::LayoutParser;

rustemo_mod!(layout, "/src/layout/generic_tree");

#[test]
fn layout_generic() {
    // ANCHOR: input
    let result = LayoutParser::new().parse("42 This6 should be 8 ignored 9 ");
    // ANCHOR_END: input
    output_cmp!(
        "src/layout/generic_tree/layout.ast",
        format!("{:#?}", result)
    )
}
