use std::io;
// Use the generated parser
use crate::calculator::CalculatorParser;

// Include generated modules
mod calculator;
#[allow(unused)]
mod calculator_actions;

#[cfg(test)]
mod tests;

fn main() {
    let mut expression = String::new();

    // Read the line from the input
    println!("Expression:");
    io::stdin().read_line(&mut expression).expect("Failed to read line.");

    // Parse the line and get the result.
    let result = CalculatorParser::parse(&expression);

    // Print the result using Debug formatter.
    println!("{:#?}", result);
}
