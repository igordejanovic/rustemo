use clap::clap_derive::ArgEnum;
use std::fs;

use crate::table::TableType;
use crate::{Error, Result};
use std::path::{Path, PathBuf};

use crate::generator::generate_parser;

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

impl Settings {
    pub fn new() -> Self {
        Settings::default()
    }
    pub fn root_dir(mut self, root_dir: PathBuf) -> Self {
        self.root_dir = Some(root_dir);
        self
    }
    pub fn out_dir_root(mut self, out_dir: PathBuf) -> Self {
        self.out_dir_root = Some(out_dir);
        self
    }
    pub fn out_dir_actions_root(mut self, out_dir: PathBuf) -> Self {
        self.out_dir_actions_root = Some(out_dir);
        self
    }
    pub fn in_source_tree(mut self) -> Self {
        self.out_dir_root = None;
        self.out_dir_actions_root = None;
        self
    }
    pub fn actions_in_source_tree(mut self) -> Self {
        if !matches!(self.builder_type, BuilderType::Default) {
            panic!("Settings 'actions_in_source_tree' is only available for the default builder type!");
        }
        self.out_dir_actions_root = None;
        self
    }
    pub fn exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = exclude;
        self
    }
    pub fn prefer_shifts(mut self, prefer: bool) -> Self {
        self.prefer_shifts = prefer;
        self
    }
    pub fn prefer_shifts_over_empty(mut self, prefer: bool) -> Self {
        self.prefer_shifts_over_empty = prefer;
        self
    }
    pub fn table_type(mut self, table_type: TableType) -> Self {
        self.table_type = table_type;
        self
    }
    pub fn parser_algo(mut self, parser_algo: ParserAlgo) -> Self {
        self.parser_algo = parser_algo;
        self
    }
    pub fn lexer_type(mut self, lexer_type: LexerType) -> Self {
        self.lexer_type = lexer_type;
        self
    }
    pub fn builder_type(mut self, builder_type: BuilderType) -> Self {
        self.builder_type = builder_type;
        self
    }
    pub fn print_table(mut self, print_table: bool) -> Self {
        self.print_table = print_table;
        self
    }
    pub fn partial_parse(mut self, partial_parse: bool) -> Self {
        self.partial_parse = partial_parse;
        self
    }
    pub fn skip_ws(mut self, skip_ws: bool) -> Self {
        self.skip_ws = skip_ws;
        self
    }
    pub fn actions(mut self, actions: bool) -> Self {
        self.actions = actions;
        self
    }
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }
    pub fn process_dir(&self) -> Result<()> {
        if let Some(root_dir) = &self.root_dir {
            if !root_dir.exists() {
                return Err(Error::Error("Folder doesn't exist.".to_string()));
            }

            let visitor = |grammar: &Path| -> Result<()> {
                self.process_grammar(grammar)?;
                Ok(())
            };

            self.visit_dirs(root_dir, &visitor)
        } else {
            Err(Error::Error("Root dir must be set!".to_string()))
        }
    }

    pub fn process_grammar(&self, grammar: &Path) -> Result<()> {
        println!("Generating parser for grammar {:?}", grammar);
        let relative_outdir = |p: &Path| -> Result<PathBuf> {
            Ok(p.join(
                grammar
                    .parent()
                    .ok_or(Error::Error("Cannot find parent of '{grammar:?}' file.".to_string()))?
                    .strip_prefix(self.root_dir.as_ref().expect("'root_dir' must be set!"))
                    .or(Err(Error::Error("Cannot remove prefix '{root_dir:?}' from '{grammar:?}'.".to_string())))?
                ))
        };

        let out_dir =
            match self.out_dir_root.as_ref().map(|p| relative_outdir(p)) {
                Some(result) => Some(result?),
                None => None,
            };

        let out_dir_actions = match self
            .out_dir_actions_root
            .as_ref()
            .map(|p| relative_outdir(p))
        {
            Some(result) => Some(result?),
            None => None,
        };

        if let Some(ref dir) = out_dir {
            println!("Parser out dir: {dir:?}");
        }
        if let Some(ref dir) = out_dir_actions {
            println!("Actions out dir: {dir:?}");
        }

        generate_parser(
            grammar,
            out_dir.as_deref(),
            out_dir_actions.as_deref(),
            &self,
        )
    }

    /// Recursively visits dirs starting from the given `dir` and calls
    /// `visitor` for each Rustemo grammar found.
    fn visit_dirs(
        &self,
        dir: &Path,
        visitor: &dyn Fn(&Path) -> Result<()>,
    ) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                // Check excluded paths
                let path_name = path.to_string_lossy();
                if self.exclude.iter().any(|e| path_name.contains(e)) {
                    println!("Excluding path: {path_name:?}");
                    continue;
                }

                if path.is_dir() {
                    self.visit_dirs(&path, visitor)?;
                } else if matches!(path.extension(), Some(ext) if ext == "rustemo")
                {
                    visitor(&path)?
                }
            }
        }
        Ok(())
    }
}

/// Recursively process a given dir and generate a parser for each found
/// grammar with default settings.
///
/// # Example
///
/// ```rust
/// rustemo_compiler::process_dir("~/my_project")
/// ```
///
/// # Errors
///
/// In case of an error a value of [rustemo::Error] is returned.
pub fn process_dir<P: AsRef<Path>>(dir: P) -> Result<()> {
    Settings::new()
        .root_dir(PathBuf::from(dir.as_ref()))
        .process_dir()?;
    Ok(())
}

/// A shortcut function which creates default [Settings] and use it to process
/// the crate project folder.
pub fn process_crate_dir() -> Result<()> {
    Settings::new().process_dir()?;
    Ok(())
}

/// Generates a parser from the given grammar file with default settings.
///
/// # Errors
///
/// In case of an error a value of [rustemo::Error] is returned.
pub fn process_grammar<P: AsRef<Path>>(grammar: P) -> Result<()> {
    Settings::new().process_grammar(grammar.as_ref())?;
    Ok(())
}
