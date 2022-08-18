use rustemo::output_cmp;
use rustemo_rt::rustemo_mod;

rustemo_mod!(reduce_empty_1, "/src/ambiguity/reduce_empty_1");
rustemo_mod!(reduce_empty_1_actions, "/src/ambiguity/reduce_empty_1");
rustemo_mod!(reduce_empty_2, "/src/ambiguity/reduce_empty_2");
rustemo_mod!(reduce_empty_2_actions, "/src/ambiguity/reduce_empty_2");

//#[test]
fn reduce_empty_1() {
    let result = ReduceEmptyParser::parse_str("b b b");
}

//#[test]
fn reduce_empty_2() {
    let result = ReduceEmptyParser::parse_str("b b b");
}
