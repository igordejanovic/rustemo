use std::process::exit;

fn main() {
    if let Err(e) = rustemo_compiler::with_settings().process_dir() {
        eprintln!("{}", e);
        exit(1);
    }
}
