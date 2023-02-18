use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;
mod pass_context_actions;

rustemo_mod!(pass_context, "/src/pass_context");

use self::pass_context::PassContextParser;

// ANCHOR: pass_context
#[test]
fn pass_context() {
    let result = PassContextParser::parse("a 1 42 b");
    output_cmp!(
        "src/pass_context/pass_context.ast",
        format!("{:#?}", result)
    );
}
// ANCHOR_END: pass_context
