use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;

use self::layout::LayoutParser;

rustemo_mod!(layout, "/src/layout");
rustemo_mod!(layout_actions, "/src/layout");

#[test]
fn layout() {
    let result = LayoutParser::parse("1 42 This 6 should be 89 ignored 8");
    output_cmp!("src/layout/layout.ast", format!("{:#?}", result))
}
