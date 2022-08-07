use std::path::PathBuf;

use clap::Parser;
use rustemo::{generator::generate_parser, settings::Settings};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Regenerate output even if exists
    #[clap(short, long, action)]
    force: bool,

    /// Rustemo grammar file to parse
    #[clap(value_parser, value_name="GRAMMAR FILE", value_hint = clap::ValueHint::FilePath)]
    grammar_file: PathBuf,

    /// Output directory. Default is the same as input grammar file.
    #[clap(short, long, value_name="OUT DIR", value_hint = clap::ValueHint::DirPath)]
    outdir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match generate_parser(cli.grammar_file, cli.outdir, &Settings::default()) {
        Ok(_) => println!("Parser generated successfully"),
        Err(e) => return Err(format!("Parser not generated. {e}").into()),
    }

    Ok(())
}
