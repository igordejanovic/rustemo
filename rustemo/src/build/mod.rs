use crate::settings::Settings;
use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generator::generate_parser;

fn visit_dirs(dir: &Path, visitor: &dyn Fn(&Path) -> Result<()>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, visitor)?;
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

/// Recurse into a given directory and generate all parsers for .rustemo grammar
/// files.
pub fn generate_parsers(
    root_dir: &Path,
    out_dir: Option<&Path>,
    out_dir_actions: Option<&Path>,
    settings: &Settings,
) -> Result<()> {
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

        let out_dir = out_dir.map(|p| relative_outdir(p));
        let out_dir_actions = out_dir_actions.map(|p| relative_outdir(p));

        if let Some(ref dir) = out_dir {
            println!("Parser out dir: {dir:?}");
        }
        if let Some(ref dir) = out_dir_actions {
            println!("Actions out dir: {dir:?}");
        }

        generate_parser(
            grammar.to_path_buf(),
            out_dir,
            out_dir_actions,
            settings,
        )
    };

    visit_dirs(&root_dir.as_ref(), &visitor)
}
