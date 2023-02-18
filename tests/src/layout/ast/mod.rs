use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;

use self::layout::LayoutParser;

rustemo_mod!(layout, "/src/layout/ast");
rustemo_mod!(layout_actions, "/src/layout/ast");

#[test]
fn layout() {
    // ANCHOR: input
    let result = LayoutParser::parse("1 42 This 6 should be 89 ignored 8");
    // ANCHOR_END: input
    output_cmp!("src/layout/ast/layout.ast", format!("{:#?}", result))
}
