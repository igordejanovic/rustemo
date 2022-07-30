use std::path::PathBuf;

use clap::Parser;
use rustemo::generator::generate_parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Regenerate output even if exists
    #[clap(short, long, action)]
    force: bool,

    /// Rustemo grammar file to parse
    #[clap(value_parser, value_name="GRAMMAR FILE", value_hint = clap::ValueHint::FilePath)]
    grammar_file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match generate_parser(cli.grammar_file) {
        Ok(_) => println!("Parser generated successfully"),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
