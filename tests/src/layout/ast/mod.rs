use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

use self::layout::LayoutParser;

rustemo_mod!(layout, "/src/layout/ast");
rustemo_mod!(layout_actions, "/src/layout/ast");

#[test]
fn layout_ast() {
    // ANCHOR: input
    let result = LayoutParser::parse("42 This6 should be 8 ignored 9 ");
    // ANCHOR_END: input
    output_cmp!("src/layout/ast/layout.ast", format!("{:#?}", result))
}
