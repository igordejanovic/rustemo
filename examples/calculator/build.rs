use std::{path::PathBuf, process::exit};

fn main() {
    let grammar_file: PathBuf = [env!("CARGO_MANIFEST_DIR"),
                                 "src", "calculator.rustemo"].iter().collect();
    if let Err(e) = rustemo::generate_parser(grammar_file) {
        eprintln!("{}", e);
        exit(1);
    }
}
