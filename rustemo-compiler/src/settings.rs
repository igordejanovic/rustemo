use clap::clap_derive::ArgEnum;

use crate::table::TableType;
use std::path::PathBuf;

#[derive(Debug, Clone, ArgEnum)]
pub enum ParserAlgo {
    LR,
    GLR,
}

impl Default for ParserAlgo {
    fn default() -> Self {
        ParserAlgo::LR
    }
}

#[derive(Debug, Clone, ArgEnum)]
pub enum LexerType {
    Default,
    Custom,
}

impl Default for LexerType {
    fn default() -> Self {
        LexerType::Default
    }
}

#[derive(Debug, Clone, ArgEnum)]
pub enum BuilderType {
    Default,
    Generic,
    Custom,
}

impl Default for BuilderType {
    fn default() -> Self {
        BuilderType::Default
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    /// Output root for the generated parser. If `None` the parser is generated
    /// in the source tree next to the grammar.
    pub out_dir_root: Option<PathBuf>,

    /// Output root for the generated actions when default builder is used. If
    /// `None` actions are generated in the source tree next to the grammar.
    pub out_dir_actions_root: Option<PathBuf>,

    /// Root dir used to calculate output file path from the input grammar path
    /// when the `out_dir_root` is not `None`.
    /// It can be overriden explicitly or when using `process_dir` call.
    /// It is an error if `root_dir` is `None`, `our_dir_root` is set an
    /// `CARGO_MANIFEST_DIR` env variable cannot be found.
    pub root_dir: Option<PathBuf>,

    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub table_type: TableType,
    pub parser_algo: ParserAlgo,
    pub print_table: bool,
    pub exclude: Vec<String>,
    pub actions: bool,

    /// What kind of lexer should be used.
    pub lexer_type: LexerType,

    /// What builder should be generated.
    pub builder_type: BuilderType,

    /// If partial parse is allowed parsing can succeed even if the parser
    /// didn't reach the end of the input. Use with care, especially with GLR
    /// parsing as it may lead to a large number of partial solutions.
    pub partial_parse: bool,

    /// Should whitespace be skipped. Not used if Layout rule exists in the Grammar.
    pub skip_ws: bool,

    /// Should actions file be recreated if exist. Use with care.
    pub force: bool,
}

impl Default for Settings {
    fn default() -> Self {
        // If called from cargo build use OUT_DIR as a default out_dir folder
        // for both parser and actions.
        let out_dir_root =
            std::env::var("OUT_DIR").map_or(None, |d| Some(PathBuf::from(d)));

        // By default root dir is the root of the cargo project.
        let root_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map_or(None, |d| Some(PathBuf::from(d)));

        Self {
            root_dir,
            out_dir_root: out_dir_root.clone(),
            out_dir_actions_root: out_dir_root,
            prefer_shifts: false,
            prefer_shifts_over_empty: true,
            table_type: Default::default(),
            parser_algo: Default::default(),
            print_table: false,
            actions: true,
            lexer_type: Default::default(),
            builder_type: Default::default(),
            partial_parse: false,
            skip_ws: true,
            force: false,
            exclude: vec![],
        }
    }
}
