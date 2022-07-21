mod calculator;

fn main() {
    println!("{:?}", calculator::CalculatorParser.parse("2 + 3".into()));
}
