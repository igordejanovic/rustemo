use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;
mod pass_context_actions;

rustemo_mod!(pass_context, "/src/pass_context");

use self::pass_context::PassContextParser;

#[test]
fn pass_context(){
    let result = PassContextParser::parse("a 1 42 b");
    output_cmp!("src/pass_context/pass_context.ast", format!("{:#?}", result));
}
