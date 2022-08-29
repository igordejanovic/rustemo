use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;

use self::layout::LayoutParser;

rustemo_mod!(layout, "/src/layout");
rustemo_mod!(layout_actions, "/src/layout_actions");

pub type Input = str;

#[test]
fn layout() {
    let result = LayoutParser::parse_str("1 42 This 6 should be 89 ignored 8");
    output_cmp!("src/layout/layout.ast", format!("{:#?}", result))
}
