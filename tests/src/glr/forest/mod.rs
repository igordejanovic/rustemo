use std::borrow::BorrowMut;

use rustemo::rustemo_mod;
use rustemo_compiler::output_cmp;

rustemo_mod!(calc, "/src/glr/forest");
rustemo_mod!(calc_actions, "/src/glr/forest");

use self::calc::CalcParser;
use rustemo::parser::Parser;

#[test]
fn glr_calc_parse_ast() {
    let result = CalcParser::new().parse("1 + 4 * 9 + 3 * 2").unwrap();
    output_cmp!("src/glr/calc.ast", format!("{:#?}", result));

    // Catalan number for 5 operands
    assert_eq!(result.solutions(), 14);

    // Number of ambiguous nodes
    println!("Solutions: {}", result.solutions());
    println!("Ambiguities: {}", result.ambiguities());
}

#[test]
fn glr_calc_parse_solutions() {
    // Number of solutions is calculated using a sequence called Catalan numbers
    // https://en.wikipedia.org/wiki/Catalan_number
    assert_eq!(CalcParser::new().parse("1 + 4 * 9").unwrap().solutions(), 2);
    assert_eq!(
        CalcParser::new()
            .parse("1 + 4 * 9 + 3")
            .unwrap()
            .solutions(),
        5
    );
    assert_eq!(
        CalcParser::new()
            .parse("1 + 4 * 9 + 3 * 2")
            .unwrap()
            .solutions(),
        14
    );
    assert_eq!(
        CalcParser::new()
            .parse("1 + 4 * 9 + 3 * 2 + 7")
            .unwrap()
            .solutions(),
        42
    );
}

/// The number of ambiguities in the forest.
#[test]
fn glr_calc_parse_ambiguities() {
    assert_eq!(
        CalcParser::new().parse("1 + 4 * 9").unwrap().ambiguities(),
        1
    );
    assert_eq!(
        CalcParser::new()
            .parse("1 + 4 * 9 + 3")
            .unwrap()
            .ambiguities(),
        3
    );
    assert_eq!(
        CalcParser::new()
            .parse("1 + 4 * 9 + 3 * 2")
            .unwrap()
            .ambiguities(),
        6
    );
}

#[test]
fn glr_extract_tree_from_forest() {
    let forest = CalcParser::new().parse("1 + 4 * 9 + 3 * 2 + 7").unwrap();
    output_cmp!(
        "src/glr/forest_tree_first.ast",
        format!("{:#?}", forest.get_first_tree().unwrap())
    );
    output_cmp!(
        "src/glr/forest_tree_17.ast",
        format!("{:#?}", forest.get_tree(17).unwrap())
    );
    output_cmp!(
        "src/glr/forest_tree_last.ast",
        format!("{:#?}", forest.get_tree(41).unwrap())
    );

    // Accessing a tree past the last.
    assert!(forest.get_tree(42).is_none());

    let tree = forest.get_tree(41).unwrap();
    output_cmp!(
        "src/glr/forest_tree_children.ast",
        format!("{:#?}", tree.children()[0].children())
    );
}
