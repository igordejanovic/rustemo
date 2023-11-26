use std::borrow::BorrowMut;

use rustemo::{rustemo_mod, Parser};
use rustemo_compiler::output_cmp;

rustemo_mod!(calc, "/src/glr/forest");
rustemo_mod!(calc_actions, "/src/glr/forest");

use self::calc::CalcParser;

#[test]
fn glr_calc_parse_ast() {
    let result = CalcParser::new().parse("1 + 4 * 9 + 3 * 2").unwrap();
    output_cmp!("src/glr/forest/calc.ast", format!("{:#?}", result));

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

// ANCHOR: forest
#[test]
fn glr_extract_tree_from_forest() {
    let forest = CalcParser::new().parse("1 + 4 * 9 + 3 * 2 + 7").unwrap();
    output_cmp!(
        "src/glr/forest/forest_tree_first.ast",
        format!("{:#?}", forest.get_first_tree().unwrap())
    );
    output_cmp!(
        "src/glr/forest/forest_tree_17.ast",
        format!("{:#?}", forest.get_tree(17).unwrap())
    );
    output_cmp!(
        "src/glr/forest/forest_tree_last.ast",
        format!("{:#?}", forest.get_tree(41).unwrap())
    );

    // Accessing a tree past the last.
    assert!(forest.get_tree(42).is_none());

    let tree = forest.get_tree(41).unwrap();
    output_cmp!(
        "src/glr/forest/forest_tree_children.ast",
        format!("{:#?}", tree.children()[0].children())
    );
}
// ANCHOR_END: forest

#[test]
fn glr_forest_into_iter() {
    let forest = CalcParser::new().parse("1 + 4 * 9 + 3 * 2 + 7").unwrap();
    let mut forest_get_tree_string = String::new();
    let mut forest_iter_string = String::new();

    for tree_idx in 0..forest.solutions() {
        forest_get_tree_string
            .push_str(&format!("{:#?}", forest.get_tree(tree_idx).unwrap()))
    }

    for tree in forest {
        forest_iter_string.push_str(&format!("{tree:#?}"));
    }
    assert_eq!(forest_get_tree_string, forest_iter_string);
    output_cmp!("src/glr/forest/forest_into_iter.ast", forest_iter_string);
}

#[test]
fn glr_forest_iter() {
    let forest = CalcParser::new().parse("1 + 4 * 9 + 3 * 2 + 7").unwrap();
    let mut forest_get_tree_string = String::new();
    let mut forest_iter_string = String::new();
    let mut forest_iter_ref_string = String::new();

    for tree_idx in 0..forest.solutions() {
        forest_get_tree_string
            .push_str(&format!("{:#?}", forest.get_tree(tree_idx).unwrap()))
    }

    for tree in forest.iter() {
        forest_iter_string.push_str(&format!("{tree:#?}"));
    }

    for tree in &forest {
        forest_iter_ref_string.push_str(&format!("{tree:#?}"));
    }
    assert_eq!(forest_get_tree_string, forest_iter_string);
    assert_eq!(forest_get_tree_string, forest_iter_ref_string);
    output_cmp!("src/glr/forest/forest_iter.ast", forest_iter_string);
}
