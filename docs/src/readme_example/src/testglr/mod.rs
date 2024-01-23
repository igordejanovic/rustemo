#[rustfmt::skip]
mod calc;
mod calc_actions;
use self::calc::{CalcParser, DefaultBuilder};
use rustemo::Parser;

#[test]
fn test_glr() {
    let forest = CalcParser::new().parse("2 + 3 * 4 + 1").unwrap();

    // We have 5 possible solutions, see https://en.wikipedia.org/wiki/Catalan_number
    assert_eq!(forest.solutions(), 5);

    // Evaluate each tree from the forest
    let results = forest
        .into_iter()
        .map(|tree| {
            let mut builder = DefaultBuilder::new();
            tree.build(&mut builder)
        })
        .collect::<Vec<_>>();

    assert_eq!(results, vec![21, 15, 25, 15, 17]);
}
