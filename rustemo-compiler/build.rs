//! Build script for bootstrapping rustemo parser.
//! Please see bootstrapping.md in the docs.
use std::error::Error;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

const PROJECT: &str = "rustemo-compiler";

fn main() {
    // Rebuild if head changed to include the new git hash.
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=src");

    if env::var("CARGO_FEATURE_BOOTSTRAP").is_ok() {
        if let Err(err) = bootstrap() {
            eprintln!("{err}");
            exit(1);
        }
    }

    let git_hash = std::env::var("GIT_HASH")
        .ok()
        .or_else(get_git_hash_if_building_compiler)
        .unwrap_or_default();

    if !git_hash.is_empty() {
        println!("cargo:rustc-env=GIT_HASH=-{}", cut_git_hash(&git_hash));
    } else {
        println!("cargo:rustc-env=GIT_HASH=");
    }
}

fn get_git_hash_if_building_compiler() -> Option<String> {
    if matches!(std::env::var("CARGO_PKG_NAME"), Ok(pkg_name) if pkg_name == "rustemo-compiler") {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        Some(String::from_utf8(output.stdout).unwrap())
    } else {
        None
    }
}

fn cut_git_hash(hash: &str) -> &str {
    const CUT_COUNT: usize = 10;
    let end_idx = hash
        .char_indices()
        .map(|x| x.0)
        .nth(CUT_COUNT)
        .unwrap_or(hash.len());
    &hash[..end_idx]
}

fn bootstrap() -> Result<(), Box<dyn Error>> {
    println!("Building bootstrap binary.");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set OUT_DIR"));

    fs::create_dir_all(out_dir.join("src/lang"))?;

    for f in [
        format!("{PROJECT}/src/lang/rustemo.rs"),
        format!("{PROJECT}/src/lang/rustemo_actions.rs"),
    ] {
        let output = Command::new("git")
            .args(["show", &format!("main:{f}")])
            .output()
            .unwrap_or_else(|_| panic!("Cannot checkout file {f:?}"));

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

        println!("{out_file:?}");

        fs::write(&out_file, output.stdout)
            .unwrap_or_else(|_| panic!("Cannot write to file {out_file:?}."));
    }

    println!("Git checkout complete!");

    Ok(())
}
