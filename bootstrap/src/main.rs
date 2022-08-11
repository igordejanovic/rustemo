use std::{process::{exit, Command}, env, path::PathBuf, fs};
use clap::Parser;

#[derive(Parser)]
#[clap(author, about, version,long_about = None)]
struct Cli {
    /// Bootstrap command. Either 'start' or 'finish'.
    #[clap(value_parser, value_name="COMMAND")]
    command: String,
}


fn main() {
    let cli = Cli::parse();
    if cli.command == "start" {
        Bootstrap::new().start();
    } else if cli.command == "finish" {
        Bootstrap::new().finish();
    } else {
        println!("You must provide either 'start' or 'finish' as a command.");
        exit(1);
    }
}

struct Bootstrap {
    parser_path: PathBuf,
    bootstrap_parser_path: PathBuf,
    actions_path: PathBuf,
    bootstrap_actions_path: PathBuf,
}

impl Bootstrap {
    fn new() -> Self {
        let root = PathBuf::from("rustemo/src/lang");
        Self {
            parser_path: root.join("rustemo.rs"),
            bootstrap_parser_path: root.join("rustemo_bootstrap.rs"),
            actions_path: root.join("rustemo_actions.rs"),
            bootstrap_actions_path: root.join("rustemo_actions_bootstrap.rs"),
        }
    }

    fn _find_project_root(&self) -> PathBuf {
        let path = env::current_dir().expect("Cannot get the current dir.");
        path.ancestors().find(|&p| {
            p.join(".git").exists() && p.join("bootstrap").exists()
        }).expect("Cannot locate project root.
                Did you start the command inside the rustemo project?")
        .to_path_buf()
    }

    fn finish(&self) {
        let root_dir = self._find_project_root();
        println!("Deleting bootstrap files.");
        for f in &[&self.bootstrap_parser_path, &self.bootstrap_actions_path] {
            let f = root_dir.join(f);
            if f.exists() {
                match fs::remove_file(&f) {
                    Err(_) => println!("Error removing file {:?}", f),
                    Ok(_) => println!("File {:?} removed.", f),
                }
            } else {
                println!("File {:?} doesn't exists.", f);
            }
        }
    }

    fn start(&self) {
        let root_dir = self._find_project_root();
        println!("Starting bootstrap process.");
        println!("Checking out parser files from the 'main' branch.");

        for (f, boot) in [(&self.parser_path, &self.bootstrap_parser_path),
                          (&self.actions_path, &self.bootstrap_actions_path)] {
            let output = Command::new("git")
                .args(&[
                    "show",
                    &format!("main:{}", f.to_str().unwrap()),
                ])
                .output().expect(&format!("Cannot checkout file {:?}", f));

            let out_file = root_dir.join(boot);
            println!("{:?}", out_file);

            fs::write(&out_file,  output.stdout)
                .expect(&format!("Cannot write to file {:?}.", out_file));
        }

        println!("Git checkout complete!");
        println!("Running 'cargo build --features bootstrap' to produce bootstrap binary.");
        let status = Command::new("cargo")
            .args(&[
                "build",
                "--manifest-path",
                root_dir.join("Cargo.toml").to_str().unwrap(),
                "--features",
                "bootstrap",
                "-p",
                "rustemo"
            ]).status().expect("Failed to execute cargo.");


        if !status.success() {
            println!("Error: failed to build bootstrap binary.")
        } else {
            println!("Bootstrapping initialization complete!");
        }
    }
}

