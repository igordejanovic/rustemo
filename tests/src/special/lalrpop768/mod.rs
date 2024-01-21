use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(lalrpop768, "/src/special/lalrpop768");
rustemo_mod!(lalrpop768_actions, "/src/special/lalrpop768");
use self::lalrpop768::Lalrpop768Parser;

#[test]
fn pager_g1() {
    let result = Lalrpop768Parser::new().parse("u x b a");

    output_cmp!(
        "src/special/lalrpop768/lalrpop768.ast",
        format!("{result:#?}")
    );
}
