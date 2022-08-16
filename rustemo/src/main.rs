use std::path::PathBuf;

use clap::Parser;
use rustemo::api::with_settings;

#[derive(Parser)]
#[cfg_attr(feature="bootstrap",
           clap(version = concat!(concat!(env!("CARGO_PKG_VERSION"), "-",
                                          env!("GIT_HASH")), "-bootstrap")))]
#[cfg_attr(not(feature="bootstrap"),
           clap(version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH"))))]
#[clap(author, about, long_about = None)]
struct Cli {
    /// Regenerate output actions file even if exists
    #[clap(short, long, action)]
    force: bool,

    /// Do not generate actions
    #[clap(short, long, action)]
    noactions: bool,

    /// Grammar file or directory to process
    #[clap(value_parser, value_name="GRAMMAR FILE/DIR", value_hint = clap::ValueHint::AnyPath)]
    grammar_file_or_dir: PathBuf,

    /// Output directory for the parser. Default is the same as input grammar file.
    #[clap(short, long, value_name="OUT DIR", value_hint = clap::ValueHint::DirPath)]
    outdir: Option<PathBuf>,

    /// Output directory for actions. Default is the same as input grammar file.
    #[clap(short='a', long, value_name="OUT DIR ACTIONS", value_hint = clap::ValueHint::DirPath)]
    outdir_actions: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let settings = with_settings()
        .force(cli.force)
        .actions(!cli.noactions);

    let result = if cli.grammar_file_or_dir.is_file() {
        settings.process_grammar(&cli.grammar_file_or_dir)
    } else {
        settings.process_dir(&cli.grammar_file_or_dir)
    };

    if let Err(e) = result {
        Err(format!("Parser not generated. {e}").into())
    } else {
        println!("Parser generated successfully");
        Ok(())
    }
}
