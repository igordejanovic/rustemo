use std::fs;
use std::path::{Path, PathBuf};

pub use crate::table::TableType;
pub use crate::{Error, Result};

pub use crate::generator::generate_parser;

pub use self::settings::{ParserAlgo, Settings};

pub(crate) mod settings;

/// A wrapper type around `Settings` used for a builder pattern style
/// configuration. It is not meant to be constructed directly but through
/// `with_settings` function.
#[derive(Debug, Clone)]
pub struct RustemoSettings(Settings);

/// Used as an entry point if non-default settings are needed.
///
/// # Example
///
/// ```rust
/// rustemo::with_settings().force(true).process_dir(some_dir);
/// ```
pub fn with_settings() -> RustemoSettings {
    RustemoSettings(Settings::default())
}

/// Recursivelly process a given dir and generate a parser for each found
/// grammar with default settings.
///
/// # Errors
///
/// In case of an error a value of `rustemo::Error` is returned.
pub fn process_dir<P: AsRef<Path>>(dir: P) -> Result<()> {
    with_settings().process_dir(dir.as_ref())?;
    Ok(())
}

/// Generates a parser from the given grammar file with default settings.
///
/// # Errors
///
/// In case of an error a value of `rustemo::Error` is returned.
pub fn process_grammar<P: AsRef<Path>>(grammar: P) -> Result<()> {
    with_settings().process_grammar(grammar.as_ref())?;
    Ok(())
}

impl RustemoSettings {
    pub fn out_dir(mut self, out_dir: Option<PathBuf>) -> Self {
        self.0.out_dir = out_dir;
        self
    }
    pub fn out_dir_actions(mut self, out_dir: Option<PathBuf>) -> Self {
        self.0.out_dir_actions = out_dir;
        self
    }
    pub fn exclude(mut self, exclude: Vec<String>) -> Self {
        self.0.exclude = exclude;
        self
    }
    pub fn prefer_shifts(mut self, prefer: bool) -> Self {
        self.0.prefer_shifts = prefer;
        self
    }
    pub fn prefer_shifts_over_empty(mut self, prefer: bool) -> Self {
        self.0.prefer_shifts_over_empty = prefer;
        self
    }
    pub fn table_type(mut self, table_type: TableType) -> Self {
        self.0.table_type = table_type;
        self
    }
    pub fn parser_algo(mut self, parser_algo: ParserAlgo) -> Self {
        self.0.parser_algo = parser_algo;
        self
    }
    pub fn partial_parse(mut self, partial_parse: bool) -> Self {
        self.0.partial_parse = partial_parse;
        self
    }
    pub fn actions(mut self, actions: bool) -> Self {
        self.0.actions = actions;
        self
    }
    pub fn pass_context(mut self, pass_context: bool) -> Self {
        self.0.pass_context = pass_context;
        self
    }
    pub fn force(mut self, force: bool) -> Self {
        self.0.force = force;
        self
    }
    pub fn process_dir(&self, root_dir: &Path) -> Result<()> {
        if !root_dir.exists() {
            return Err(Error::Error("Folder doesn't exist.".to_string()));
        }
        let visitor = |grammar: &Path| -> Result<()> {
            println!("Generating parser for grammar {:?}", grammar);

            let relative_outdir = |p: &Path| -> PathBuf {
                p.join(
                    grammar
                        .parent()
                        .expect(&format!("Cannot find parent of '{grammar:?}' file."))
                        .strip_prefix(root_dir)
                        .expect(&format!(
                            "Cannot remove prefix '{root_dir:?}' from '{grammar:?}'.")),
                    )
            };

            let out_dir = self.0.out_dir.as_ref().map(|p| relative_outdir(&p));
            let out_dir_actions =
                self.0.out_dir_actions.as_ref().map(|p| relative_outdir(&p));

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
                &self.0,
            )
        };

        self.visit_dirs(&root_dir.as_ref(), &visitor)
    }

    pub fn process_grammar(&self, grammar_file: &Path) -> Result<()> {
        generate_parser(
            grammar_file,
            self.0.out_dir.as_deref(),
            self.0.out_dir_actions.as_deref(),
            &self.0,
        )
    }

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
                if self.0.exclude.iter().any(|e| path_name.contains(e)) {
                    println!("Excluding path: {path_name:?}");
                    continue;
                }

                if path.is_dir() {
                    self.visit_dirs(&path, visitor)?;
                } else {
                    match path.extension() {
                        Some(ext) if ext == "rustemo" => visitor(&path)?,
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }
}
