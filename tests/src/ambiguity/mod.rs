use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;

use self::reduce_empty_1::ReduceEmpty1Parser;
use self::reduce_empty_2::ReduceEmpty2Parser;

rustemo_mod!(reduce_empty_1, "/src/ambiguity");
rustemo_mod!(reduce_empty_1_actions, "/src/ambiguity");
rustemo_mod!(reduce_empty_2, "/src/ambiguity");
rustemo_mod!(reduce_empty_2_actions, "/src/ambiguity");

#[test]
fn reduce_empty_1() {
    let result = ReduceEmpty1Parser::parse("b b b");
    output_cmp!("src/ambiguity/reduce_empty_1.ast", format!("{:#?}", result));
}

#[test]
fn reduce_empty_2() {
    let result = ReduceEmpty2Parser::parse("1 42 2 b");
    output_cmp!("src/ambiguity/reduce_empty_2.ast", format!("{:#?}", result));
}
