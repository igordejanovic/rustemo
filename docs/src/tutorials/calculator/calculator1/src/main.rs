// ANCHOR: header
use rustemo::Parser;
use std::io;
// Use the generated parser
use crate::calculator::CalculatorParser;

// Include generated modules
#[rustfmt::skip]
mod calculator;
#[allow(unused)]
#[rustfmt::skip]
mod calculator_actions;
// ANCHOR_END: header

#[cfg(test)]
mod tests;

// ANCHOR: main
fn main() {
    let mut expression = String::new();

    // Read the line from the input
    println!("Expression:");
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    // Parse the line and get the result.
    let result = CalculatorParser::new().parse(&expression);

    // Print the result using Debug formatter.
    println!("{result:#?}");
}
// ANCHOR_END: main
