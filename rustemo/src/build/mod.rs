use std::io;
use std::fs;
use std::path::{Path, PathBuf};

use crate::generator::generate_parser;

fn visit_dirs<P, F>(dir: P, visitor: F) -> io::Result<()>
where
    P: AsRef<Path>,
    F: Fn(&PathBuf) -> io::Result<()>
{
    if dir.as_ref().is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, &visitor)?;
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

pub fn generate_parsers<P: AsRef<Path>>(dir: P) -> io::Result<()> {
   visit_dirs(dir, |path| {
       generate_parser(path)
   })
}
