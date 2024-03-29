use expressions::ExpressionsParser;
use rustemo::rustemo_mod;
use rustemo::Parser;

rustemo_mod! {#[allow(unused_imports)]
pub(crate) expressions, "/src"}

fn main() {
    println!("{:#?}", ExpressionsParser::new().parse("3 + 2 + 1"));
}
