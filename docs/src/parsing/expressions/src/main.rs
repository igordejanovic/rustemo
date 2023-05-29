use expressions::ExpressionsParser;
use rustemo::rustemo_mod;

rustemo_mod! {#[allow(unused_imports)]
              pub(crate) expressions, "/src"}

rustemo_mod! {#[allow(dead_code)]
              pub(crate) expressions_actions, "/src"}

fn main() {
    println!("{:#?}", ExpressionsParser::new().parse("3 + 2 + 1"));
}
