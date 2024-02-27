use rustemo::{Parser, rustemo_mod};

rustemo_mod!(c, "/src/");
rustemo_mod!(c_actions, "/src/");

use c::CParser;

fn main() {
    let mut parser = CParser::new();
    let forest = parser.parse_file("binary_tree.c");
    println!("{forest:?}");
}
