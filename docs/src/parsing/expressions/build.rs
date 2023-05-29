use std::process::exit;

use rustemo_compiler::BuilderType;

fn main() {
    if let Err(e) = rustemo_compiler::Settings::new()
        .builder_type(BuilderType::Generic)
        .process_dir()
    {
        eprintln!("{}", e);
        exit(1);
    }
}
