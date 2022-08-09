/// Build script for bootstrapping rustemo parser.
/// Based on LALRPOP build script.
/// https://github.com/lalrpop/lalrpop/blob/master/lalrpop/build.rs
/// Please see CONTRIBUTE.md for the description of the bootstrapping process.
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

fn main() {
    // If bootstrap files exist we are in bootstrapping mode.
    if _root_dir().join("rustemo/src/lang/rustemo_bootstrap.rs").exists() {
        println!(r#"cargo:rustc-cfg=bootstrap"#);

        // Generate parser if bootstrap feature is not given.
        // bootstrap feature is used to build a bootstrapping binary only.
        if env::var("CARGO_FEATURE_BOOTSTRAP").is_err() {
            if let Err(err) = bootstrap() {
                eprintln!("{}", err);
                exit(1);
            }
        }
    } else {
        if env::var("CARGO_FEATURE_BOOTSTRAP").is_ok() {
            panic!("Using 'bootstrap' feature without initiating the bootstrap mode.")
        }
    }
}

fn find_rustemo_binary(prefix: &PathBuf) -> Option<PathBuf> {
    let rustemo_path = prefix
        .join("target")
        .join(env::var("PROFILE").unwrap())
        .join("rustemo")
        .with_extension(env::consts::EXE_EXTENSION);
    if rustemo_path.exists() {
        Some(rustemo_path)
    } else {
        println!("Trying to find rustemo binary at path {:?}", rustemo_path);
        None
    }
}

fn _root_dir() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR")
            .expect("cargo did not set CARGO_MANIFEST_DIR"),
    )
    .join("..")
}

fn bootstrap() -> Result<(), Box<dyn Error>> {
    println!("Bootstrapping parser.");

    let grammar_file = "src/lang/rustemo.rustemo";
    println!(r#"cargo:rerun-if-changed={}"#, grammar_file);

    let root_dir = _root_dir();

    let rustemo_path = find_rustemo_binary(&root_dir).unwrap_or_else(|| {
        panic!(
            "Can't find a rustemo binary to use for bootstrapping. \
             Make sure it is built and exists at target/{}/rustemo!",
            env::var("PROFILE").unwrap()
        )
    });

    let status = Command::new(&rustemo_path)
        .args(&[
            "--force",
            root_dir
                .join("rustemo")
                .join(grammar_file)
                .to_str()
                .expect("output path is not valid UTF-8"),
        ])
        .status()?;

    if !status.success() {
        // Abruptly finish build process in bootstrap mode
        // FIXME: Should return Error but it doesn't terminate the build process.
        panic!("Rustemo parser not generated! {}", status);
    }

    Ok(())
}
