use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;
mod use_context_actions;

rustemo_mod!(use_context, "/src/use_context");

use self::use_context::UseContextParser;

// ANCHOR: use_context
#[test]
fn use_context() {
    let result = UseContextParser::parse("a 1 42 b");
    output_cmp!("src/use_context/use_context.ast", format!("{:#?}", result));
}
// ANCHOR_END: use_context
