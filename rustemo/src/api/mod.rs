use crate::table::TableType;
use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generator::generate_parser;

use self::settings::Settings;

pub(crate) mod settings;

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
    pub fn out_dir(mut self, out_dir: PathBuf) -> Self {
        self.0.out_dir = Some(out_dir);
        self
    }
    pub fn out_dir_actions(mut self, out_dir: PathBuf) -> Self {
        self.0.out_dir_actions = Some(out_dir);
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
    pub fn actions(mut self, actions: bool) -> Self {
        self.0.actions = actions;
        self
    }
    pub fn force(mut self, force: bool) -> Self {
        self.0.force = force;
        self
    }
    pub fn process_dir(&self, root_dir: &Path) -> Result<()> {
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

        Self::visit_dirs(&root_dir.as_ref(), &visitor)
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
        dir: &Path,
        visitor: &dyn Fn(&Path) -> Result<()>,
    ) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    Self::visit_dirs(&path, visitor)?;
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