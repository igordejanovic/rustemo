use std::process::exit;

fn main() {
    if let Err(e) = rustemo_compiler::process_crate_dir() {
        eprintln!("{}", e);
        exit(1);
    }
}
