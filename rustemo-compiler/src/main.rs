//! Rustemo compiler. Run `rcomp --help` for more information.

use std::path::PathBuf;

use clap::Parser;
use rustemo_compiler::{
    BuilderType, LexerType, ParserAlgo, Settings, TableType,
};

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

    /// Output root directory for the parser. Default is the same as input grammar file.
    #[clap(short, long, value_name="OUT DIR ROOT", value_hint = clap::ValueHint::DirPath)]
    outdir_root: Option<PathBuf>,

    /// Output directory for actions. Default is the same as input grammar file.
    #[clap(short='a', long, value_name="OUT DIR ACTIONS ROOT", value_hint = clap::ValueHint::DirPath)]
    outdir_actions_root: Option<PathBuf>,

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

    /// What kind of lexer should be used.
    #[clap(short, long, arg_enum, default_value_t)]
    lexer_type: LexerType,

    /// Generated builder type.
    #[clap(short, long, arg_enum, default_value_t)]
    builder_type: BuilderType,

    /// Parser can succeed without consuming the whole input.
    #[clap(long)]
    partial_parse: bool,

    /// Should whitespace be skipped. Not used if Layout rule exists in the Grammar.
    #[clap(long)]
    no_skip_ws: bool,

    /// Print LR table
    #[clap(long)]
    print_table: bool,

    /// Exclude dirs containing these parts. Used with dir processing.
    #[clap(short, long, value_parser)]
    exclude: Vec<String>,

    /// Verbosity level 0-2
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
}

fn main() {
    let cli = Cli::parse();

    let mut settings = Settings::new()
        .force(cli.force)
        .actions(!cli.noactions)
        .exclude(cli.exclude)
        .prefer_shifts(cli.prefer_shifts)
        .prefer_shifts_over_empty(!cli.no_shifts_over_empty)
        .partial_parse(cli.partial_parse)
        .skip_ws(!cli.no_skip_ws)
        .table_type(cli.table_type)
        .print_table(cli.print_table)
        .parser_algo(cli.parser_algo)
        .lexer_type(cli.lexer_type)
        .builder_type(cli.builder_type);

    if let Some(outdir_root) = cli.outdir_root {
        settings = settings.out_dir_root(outdir_root);
    }
    if let Some(outdir_actions_root) = cli.outdir_actions_root {
        settings = settings.out_dir_actions_root(outdir_actions_root);
    }

    let result = if cli.grammar_file_or_dir.is_file() {
        settings.process_grammar(&cli.grammar_file_or_dir)
    } else {
        settings.root_dir(cli.grammar_file_or_dir).process_dir()
    };

    if let Err(e) = result {
        println!("{e}");
        println!("Parser(s) not generated.");
    }
}
