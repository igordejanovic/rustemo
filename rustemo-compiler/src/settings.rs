use clap::clap_derive::ArgEnum;
use std::fs;

use crate::table::TableType;
use crate::{Error, Result};
use std::path::{Path, PathBuf};

use crate::generator::generate_parser;

/// The parsing algorithm used
#[derive(Debug, Default, Clone, ArgEnum)]
pub enum ParserAlgo {
    #[default]
    LR,
    GLR,
}

/// The lexer type used during parsing to break the input into tokens
#[derive(Debug, Default, Clone, ArgEnum)]
pub enum LexerType {
    /// Default lexer if the input is `str` is based on string/regex recognizers
    #[default]
    Default,
    /// The lexer will be supplied by the user
    Custom,
}

/// The builder type used during parsing to construct the output
#[derive(Debug, Default, Clone, ArgEnum)]
pub enum BuilderType {
    /// Default builder type constructs AST using inferred node types
    #[default]
    Default,
    /// Generic builder generates CST where each node is `TreeNode`
    Generic,
    /// The builder is user provided
    Custom,
}

/// Different generated parser table variants with different trade-offs
#[derive(Debug, Default, Clone, ArgEnum)]
pub enum GeneratorTableType {
    /// Table is generated as nested static arrays
    /// Access time should be relatively good but it produces
    /// larger executables.
    Arrays,
    /// Table is an array of function pointers where functions contain match
    /// expression for further disambiguation. Uses less statically allocated
    /// storage but requires function call and pattern matching.
    #[default]
    Functions,
}

/// Provides parser settings information. It is the main entry point in the
/// parser generation process. It is meant to be used from the project
/// `build.rs` script. See [tests crate `build.rs`
/// script](https://github.com/igordejanovic/rustemo/blob/main/tests/build.rs)
/// for examples of various configurations.
///
/// The first step is to create default `Settings` instance, do necessary
/// configuration by calling methods in a builder (chain) style and, at the end,
/// call the method to process the grammar, either by directly specifying the
/// file or recursivelly processing the directory.
///
/// Most of these settings are also exposed through `rcomp` CLI tool so you can
/// process grammar and generate parsers from the command line (or shell script)
/// if you prefer.
///
/// You can read more in the [Rustemo book](https://www.igordejanovic.net/rustemo/)
///
/// ## Example
///
/// ```rust
/// rustemo_compiler::Settings::new().parser_algo(ParserAlgo::GLR).process_crate_dir()
/// ```
#[derive(Debug, Clone)]
pub struct Settings {
    pub(crate) out_dir_root: Option<PathBuf>,
    pub(crate) out_dir_actions_root: Option<PathBuf>,
    pub(crate) root_dir: Option<PathBuf>,

    pub(crate) prefer_shifts: bool,
    pub(crate) prefer_shifts_over_empty: bool,
    pub(crate) table_type: TableType,
    pub(crate) parser_algo: ParserAlgo,
    pub(crate) print_table: bool,
    pub(crate) exclude: Vec<String>,
    pub(crate) actions: bool,
    pub(crate) notrace: bool,

    pub(crate) lexer_type: LexerType,
    pub(crate) builder_type: BuilderType,
    pub(crate) generator_table_type: GeneratorTableType,
    pub(crate) input_type: String,

    pub(crate) lexical_disamb_most_specific: bool,
    pub(crate) lexical_disamb_longest_match: bool,
    pub(crate) lexical_disamb_grammar_order: bool,

    pub(crate) partial_parse: bool,
    pub(crate) skip_ws: bool,

    pub(crate) force: bool,
    force_explicit: bool,

    pub(crate) dot: bool,
    pub(crate) fancy_regex: bool,
}

impl Default for Settings {
    fn default() -> Self {
        // If called from cargo build use OUT_DIR as a default out_dir directory
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
            notrace: false,
            lexer_type: Default::default(),
            builder_type: Default::default(),
            generator_table_type: Default::default(),
            input_type: "str".into(),
            lexical_disamb_most_specific: true,
            lexical_disamb_longest_match: true,
            lexical_disamb_grammar_order: true,
            partial_parse: false,
            skip_ws: true,
            force: true, // Overwriting actions by default
            force_explicit: false,
            exclude: vec![],
            dot: false,
            fancy_regex: false,
        }
    }
}

impl Settings {
    /// Creates a default instance.
    pub fn new() -> Self {
        Settings::default()
    }

    /// Root dir used to calculate output file path from the input grammar path
    /// when the `out_dir_root` is not `None`.
    /// It can be overridden explicitly or when using `process_dir` call.
    /// It is an error if `root_dir` is `None`, `our_dir_root` is set and
    /// `CARGO_MANIFEST_DIR` env variable cannot be found.
    pub fn root_dir(mut self, root_dir: PathBuf) -> Self {
        self.root_dir = Some(root_dir);
        self
    }

    /// Sets output root for the generated parser. By default, the parser is
    /// generated in the source tree next to the grammar.
    pub fn out_dir_root(mut self, out_dir: PathBuf) -> Self {
        self.out_dir_root = Some(out_dir);
        self
    }

    /// Output root for the generated actions when default builder is used. By
    /// default, actions are generated in the source tree next to the grammar.
    pub fn out_dir_actions_root(mut self, out_dir: PathBuf) -> Self {
        self.out_dir_actions_root = Some(out_dir);
        self
    }

    /// Generate both parser and actions (for default builder) in the source
    /// tree, next to the grammar. By default, parser and actions are generated
    /// in out `OUT_DIR`.
    pub fn in_source_tree(mut self) -> Self {
        self.out_dir_root = None;
        if matches!(self.builder_type, BuilderType::Default) {
            self.actions_in_source_tree()
        } else {
            self
        }
    }

    /// Generate actions in the source tree (if the default builder is used),
    /// next to the grammar. By default, actions are generated in out `OUT_DIR`.
    pub fn actions_in_source_tree(mut self) -> Self {
        if !matches!(self.builder_type, BuilderType::Default) {
            panic!("Settings 'actions_in_source_tree' is only available for the default builder type!");
        }
        self.out_dir_actions_root = None;
        if !self.force_explicit {
            self.force = false;
        }
        self
    }

    /// Excludes path from processing. If path contains any of the string given
    /// in `exclude` vector it will be skipped.
    pub fn exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = exclude;
        self
    }

    /// When there are competing REDUCE and SHIFT operations, this settings will
    /// always favor SHIFT.
    pub fn prefer_shifts(mut self, prefer: bool) -> Self {
        self.prefer_shifts = prefer;
        self
    }

    /// When there are competing EMPTY reduction and SHIFT, this settings will
    /// always favor SHIFT.
    pub fn prefer_shifts_over_empty(mut self, prefer: bool) -> Self {
        self.prefer_shifts_over_empty = prefer;
        self
    }

    /// LR table type to construct.
    pub fn table_type(mut self, table_type: TableType) -> Self {
        self.table_type = table_type;
        self
    }

    /// LR algorithm to use
    pub fn parser_algo(mut self, parser_algo: ParserAlgo) -> Self {
        match parser_algo {
            ParserAlgo::LR => {}
            ParserAlgo::GLR => {
                // For GLR we are using RN tables
                self.table_type = TableType::LALR_RN;
                // For GLR we should not favour shifts at all
                self.prefer_shifts = false;
                self.prefer_shifts_over_empty = false;
                // We don't use grammar order by default
                self.lexical_disamb_grammar_order = false;
            }
        }
        self.parser_algo = parser_algo;
        self
    }

    /// Sets lexer type. Default lexer is used for string inputs and is based on
    /// regex/string matches from the grammar.
    pub fn lexer_type(mut self, lexer_type: LexerType) -> Self {
        self.lexer_type = lexer_type;
        self
    }

    /// Sets builder type. The default builder will deduce AST types and actions.
    pub fn builder_type(mut self, builder_type: BuilderType) -> Self {
        self.builder_type = builder_type;
        self
    }

    /// Sets generator table type. The default is nested static arrays.
    pub fn generator_table_type(
        mut self,
        generator_table_type: GeneratorTableType,
    ) -> Self {
        self.generator_table_type = generator_table_type;
        self
    }

    /// Sets the input type. Default is `str`
    pub fn input_type(mut self, input_type: String) -> Self {
        self.input_type = input_type;
        self
    }

    /// Lexical disambiguation using most specific match strategy.
    pub fn lexical_disamb_most_specific(mut self, most_specific: bool) -> Self {
        self.lexical_disamb_most_specific = most_specific;
        self
    }

    /// Lexical disambiguation using longest match strategy.
    pub fn lexical_disamb_longest_match(mut self, longest_match: bool) -> Self {
        self.lexical_disamb_longest_match = longest_match;
        self
    }

    /// Lexical disambiguation using grammar order.
    pub fn lexical_disamb_grammar_order(mut self, grammar_order: bool) -> Self {
        if let ParserAlgo::LR = self.parser_algo {
            if !grammar_order {
                panic!("Can't disable grammar order strategy for LR.")
            }
        }
        self.lexical_disamb_grammar_order = grammar_order;
        self
    }

    /// Set whether or not we use [`fancy_regex`](https://docs.rs/fancy-regex/latest/fancy_regex/)
    /// instead of [`regex`](https://docs.rs/regex/latest/regex/)
    pub fn fancy_regex(mut self, fancy_regex: bool) -> Self {
        self.fancy_regex = fancy_regex;
        self
    }

    pub fn print_table(mut self, print_table: bool) -> Self {
        self.print_table = print_table;
        self
    }

    /// If partial parse is allowed parsing can succeed even if the parser
    /// didn't reach the end of the input. Use with care, especially with GLR
    /// parsing as it may lead to a large number of partial solutions.
    pub fn partial_parse(mut self, partial_parse: bool) -> Self {
        self.partial_parse = partial_parse;
        self
    }

    /// Should whitespaces be skipped. `true` by default. Not used if Layout
    /// rule exists in the Grammar. Used only in the default lexer.
    pub fn skip_ws(mut self, skip_ws: bool) -> Self {
        self.skip_ws = skip_ws;
        self
    }

    /// Should actions be generated. `true` by default. Used only if default
    /// builder is used.
    pub fn actions(mut self, actions: bool) -> Self {
        self.actions = actions;
        self
    }

    /// Should trace log be printed. `false` by default. Does nothing for
    /// release builds as trace is only available in debug build. Can also be
    /// set by `RUSTEMO_NOTRACE=1` env variable.
    pub fn notrace(mut self, notrace: bool) -> Self {
        let notrace = if !notrace {
            std::env::var("RUSTEMO_NOTRACE").is_ok()
        } else {
            std::env::set_var("RUSTEMO_NOTRACE", "1");
            true
        };

        self.notrace = notrace;
        self
    }

    /// Should actions file be recreated if exist. Use with care.
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self.force_explicit = true;
        self
    }

    /// If this is set a .dot file with automata visualization will be produced during
    /// compiling.
    pub fn dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// Recursively traverse the root dir and process each Rustemo grammar found.
    /// Used as the last call to the configured [Settings] value.
    pub fn process_dir(&self) -> Result<()> {
        if let Some(root_dir) = &self.root_dir {
            if !root_dir.exists() {
                return Err(Error::Error(format!(
                    "Directory/File {root_dir:?} doesn't exist."
                )));
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

    /// Process the given grammar and generates the parser and actions (if
    /// default builder is used). Used as the last call to the configured
    /// [Settings] value.
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

        let out_dir = self
            .out_dir_root
            .as_ref()
            .map(|p| relative_outdir(p))
            .transpose()?;

        let out_dir_actions = self
            .out_dir_actions_root
            .as_ref()
            .map(|p| relative_outdir(p))
            .transpose()?;

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
            self,
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
/// the crate project directory.
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
