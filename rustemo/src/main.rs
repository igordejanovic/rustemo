use std::path::PathBuf;

use clap::Parser;
use rustemo::api::{with_settings, ParserAlgo, TableType};

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

    /// Prefer shifts in case of possible shift/reduce conflicts.
    #[clap(long)]
    prefer_shifts: bool,

    /// Do not prefer shifts over empty reductions.
    #[clap(long)]
    no_shifts_over_empty: bool,

    /// The type of LR table
    #[clap(short, long, arg_enum, default_value_t)]
    table_type: TableType,

    /// Parser algorithm
    #[clap(short, long, arg_enum, default_value_t)]
    parser_algo: ParserAlgo,

    /// Parser can succeed without consuming the whole input.
    #[clap(long)]
    partial_parse: bool,

    /// Should parse context be passed to actions if AST output is generated.
    #[clap(long)]
    pass_context: bool,

    /// Output directory for actions. Default is the same as input grammar file.
    #[clap(short='a', long, value_name="OUT DIR ACTIONS", value_hint = clap::ValueHint::DirPath)]
    outdir_actions: Option<PathBuf>,

    /// Exclude dirs containing these parts. Used with dir processing.
    #[clap(short, long, value_parser)]
    exclude: Vec<String>,

    /// Verbosity level 0-2
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
}

fn main() {
    let cli = Cli::parse();

    let settings = with_settings()
        .force(cli.force)
        .actions(!cli.noactions)
        .exclude(cli.exclude)
        .prefer_shifts(cli.prefer_shifts)
        .prefer_shifts_over_empty(!cli.no_shifts_over_empty)
        .partial_parse(cli.partial_parse)
        .pass_context(cli.pass_context)
        .table_type(cli.table_type)
        .parser_algo(cli.parser_algo)
        .out_dir(cli.outdir)
        .out_dir_actions(cli.outdir_actions);

    let result = if cli.grammar_file_or_dir.is_file() {
        settings.process_grammar(&cli.grammar_file_or_dir)
    } else {
        settings.process_dir(&cli.grammar_file_or_dir)
    };

    if let Err(e) = result {
        println!("{e}");
        println!("Parser(s) not generated.");
    }
}
