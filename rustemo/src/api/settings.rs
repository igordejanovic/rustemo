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

#[derive(Debug, Clone)]
pub struct Settings {
    pub out_dir: Option<PathBuf>,
    pub out_dir_actions: Option<PathBuf>,
    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub table_type: TableType,
    pub parser_algo: ParserAlgo,
    pub exclude: Vec<String>,
    pub actions: bool,

    /// If partial parse is allowed parsing can succeed even if the parser
    /// didn't reach the end of the input. Use with care, especially with GLR
    /// parsing as it may lead to a large number of partial solutions.
    pub partial_parse: bool,

    // /// Should parse context be passed to actions if AST output is generated.
    // pub pass_context: bool,
    /// Should actions file be recreated if exist. Use with care.
    pub force: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            out_dir: None,
            out_dir_actions: None,
            prefer_shifts: false,
            prefer_shifts_over_empty: true,
            table_type: Default::default(),
            parser_algo: Default::default(),
            actions: true,
            partial_parse: false,
            force: false,
            exclude: vec![],
        }
    }
}
