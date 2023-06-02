//! Build script for bootstrapping rustemo parser.
//! Please see bootstrapping.md in the docs.
use std::error::Error;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

const PROJECT: &str = "rustemo-compiler";

fn main() {
    // Rebuild if head changed to include the new git hash.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=src");

    if env::var("CARGO_FEATURE_BOOTSTRAP").is_ok() {
        if let Err(err) = bootstrap() {
            eprintln!("{}", err);
            exit(1);
        }
    }

    // If we are building this crate from source set git hash as a part of the version
    let git_hash = if matches!(std::env::var("CARGO_PKG_NAME"), Ok(pkg_name) if pkg_name == "rustemo-compiler")
    {
        // Setting environment with current head git hash to include in the version.
        // See: https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        String::from_utf8(output.stdout)
            .unwrap()
            .chars()
            .take(10)
            .collect::<String>()
    } else {
        "".into()
    };

    println!("cargo:rustc-env=GIT_HASH=-{}", git_hash);
}

fn bootstrap() -> Result<(), Box<dyn Error>> {
    println!("Building bootstrap binary.");

    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    fs::create_dir_all(out_dir.join("src/lang"))?;

    for f in [
        format!("{PROJECT}/src/lang/rustemo.rs"),
        format!("{PROJECT}/src/lang/rustemo_actions.rs"),
    ] {
        let output = Command::new("git")
            .args(["show", &format!("main:{}", f)])
            .output()
            .unwrap_or_else(|_| panic!("Cannot checkout file {:?}", f));

        if !output.status.success() {
            panic!(
                "git command execution failed! Exit status = {:?}",
                output.status
            );
        }

        let out_file = out_dir.join(
            PathBuf::from(f)
                .strip_prefix(format!("{PROJECT}/"))
                .unwrap(),
        );

        println!("{:?}", out_file);

        fs::write(&out_file, output.stdout)
            .unwrap_or_else(|_| panic!("Cannot write to file {:?}.", out_file));
    }

    println!("Git checkout complete!");

    Ok(())
}
