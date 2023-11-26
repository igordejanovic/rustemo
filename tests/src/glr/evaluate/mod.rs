use std::borrow::BorrowMut;

use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(calc, "/src/glr/evaluate");
mod calc_actions;
rustemo_mod!(calc_eval, "/src/glr/evaluate");
mod calc_eval_actions;

use self::calc::CalcParser;
use self::calc_eval::CalcEvalParser;

#[test]
fn glr_tree_calc_eval() {
    let input = "1 + 4 * 9 + 3 * 2 + 7";
    let forest = CalcParser::new().parse(input).unwrap();
    let forest_eval = CalcEvalParser::new().parse(input).unwrap();
    assert_eq!(forest.solutions(), 42);
    assert_eq!(forest_eval.solutions(), 42);

    let mut res = vec![];
    let mut builder = calc_eval::DefaultBuilder::new();
    for i in 0..42 {
        res.push((
            forest.get_tree(i).unwrap(),
            forest_eval.get_tree(i).unwrap().build(&mut builder),
        ));
    }

    output_cmp!("src/glr/evaluate/forest_eval.ast", format!("{:#?}", res));
}
