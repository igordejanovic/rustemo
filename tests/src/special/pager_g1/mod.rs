use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(pager_g1, "/src/special/pager_g1");
rustemo_mod!(pager_g1_actions, "/src/special/pager_g1");
use self::pager_g1::PagerG1Parser;

#[test]
fn pager_g1() {
    let result = PagerG1Parser::new().parse("b e e c");

    output_cmp!("src/special/pager_g1/pager_g1.ast", format!("{result:#?}"));
}
