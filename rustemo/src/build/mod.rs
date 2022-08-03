use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generator::generate_parser;

fn visit_dirs<P>(
    dir: P,
    visitor: &dyn Fn(&PathBuf) -> Result<()>,
) -> Result<()>
where
    P: AsRef<Path>,
{
    if dir.as_ref().is_dir() {
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
pub fn generate_parsers<P>(dir: P) -> Result<()>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    fn visitor(grammar: &PathBuf) -> Result<()> {
        log!("Generating parser for grammar {:?}", grammar);
        generate_parser(grammar, None)
    }

    visit_dirs(dir, &visitor)
}
