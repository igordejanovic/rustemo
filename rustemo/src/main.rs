use std::path::PathBuf;

use clap::Parser;
use rustemo::{generator::generate_parser, settings::Settings};

#[derive(Parser)]
#[cfg_attr(feature="bootstrap",
           clap(version = concat!(concat!(env!("CARGO_PKG_VERSION"), "-",
                                          env!("GIT_HASH")), "-bootstrap")))]
#[cfg_attr(not(feature="bootstrap"),
           clap(version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"))))]
#[clap(author, about, long_about = None)]
struct Cli {
    /// Regenerate output even if exists
    #[clap(short, long, action)]
    force: bool,

    /// Do not generate actions
    #[clap(short, long, action)]
    noactions: bool,

    /// Rustemo grammar file to parse
    #[clap(value_parser, value_name="GRAMMAR FILE", value_hint = clap::ValueHint::FilePath)]
    grammar_file: PathBuf,

    /// Output directory. Default is the same as input grammar file.
    #[clap(short, long, value_name="OUT DIR", value_hint = clap::ValueHint::DirPath)]
    outdir: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let settings = Settings::default()
        .with_force(cli.force)
        .with_actions(!cli.noactions);

    match generate_parser(cli.grammar_file, cli.outdir, &settings) {
        Ok(_) => println!("Parser generated successfully"),
        Err(e) => return Err(format!("Parser not generated. {e}").into()),
    }

    Ok(())
}
