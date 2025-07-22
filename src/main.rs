//! cargo-templated-examples
//!
//! Install using `cargo install cargo-templated-examples`
//!
//! Run using: `cargo templated-examples`
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

mod cargo_toml;
mod command_line;
mod commands;
mod parsing;
mod rust_file;
use cargo_toml::join;
use commands::{BuildType, CargoCommand};

use std::{
    collections::HashMap,
    fs,
    path::Path,
    process::{Command, ExitCode},
};

/// Number of passing and failing examples
struct RunOutcomes {
    passes: usize,
    fails: usize,
}

impl RunOutcomes {
    /// Create new
    fn new() -> Self {
        Self {
            passes: 0,
            fails: 0,
        }
    }

    /// Add
    fn add(&mut self, other: &RunOutcomes) {
        self.passes += other.passes;
        self.fails += other.fails;
    }
}

/// Get example command for a file
fn get_example_command(dir: &impl AsRef<Path>, eg: &str) -> CargoCommand {
    let file_command = rust_file::load_command(dir, eg);
    let cargo_toml_command = cargo_toml::load_command(dir, eg);

    // Return command
    if let Some(c) = file_command {
        if let Some(c2) = cargo_toml_command
            && c != c2
        {
            panic!("Commands set in file and Cargo.toml do not match for example \"{eg}\"");
        }
        c
    } else if let Some(c) = cargo_toml_command {
        c
    } else {
        CargoCommand::new(String::from(eg))
    }
}

/// Run an example
fn run_example(example: &str) -> Result<(), &str> {
    #[cfg(target_os = "windows")]
    let mut shell = Command::new("cmd /C");
    #[cfg(target_os = "windows")]
    shell.arg("/C");

    #[cfg(not(target_os = "windows"))]
    let mut shell = Command::new("sh");
    #[cfg(not(target_os = "windows"))]
    shell.arg("-c");
    shell.arg(example);

    let mut child = shell.spawn().expect("Error initialising example run");
    match child.wait().expect("Error running example").code() {
        Some(0) => Ok(()),
        _ => Err("Example run failed"),
    }
}

/// Run all examples in a directory
fn run_all_examples(dir: &Path, package: Option<String>) -> RunOutcomes {
    let mut outcomes = RunOutcomes::new();

    if let Some(w) = cargo_toml::get_workspace(&dir) {
        for c in w {
            outcomes.add(&run_all_examples(&join(&dir, &c), Some(c)));
        }
    }

    let default_build = cargo_toml::get_default_build(&dir);

    if !join(&dir, "examples").is_dir() {
        return outcomes;
    }

    // Load all template examples from files
    let mut examples = vec![];
    for file in fs::read_dir(join(&dir, "examples")).expect("Could not find examples directory") {
        let file = file.expect("Error reading examples directory").path();
        if let Some(e) = file.extension()
            && e == "rs"
        {
            let file_stem = file
                .file_stem()
                .expect("Error parsing file name")
                .to_str()
                .expect("Error parsing file name");

            let mut c = get_example_command(&dir, file_stem);
            c.set_default_build_type(&default_build);
            c.set_required_features(&cargo_toml::load_required_features(&dir, file_stem));
            if let Some(p) = &package {
                c.set_package(p);
            }
            examples.push(c.as_string());
        }
    }

    // Substitute all template arguments
    let mut template_args = HashMap::new();
    cargo_toml::load_args(&dir, &mut template_args);
    command_line::load_args(&mut template_args);

    for (a, options) in &template_args {
        let mut new_examples = vec![];
        let a = format!("{{{{{a}}}}}");
        for c in &examples {
            if c.contains(&a) {
                for o in options {
                    new_examples.push(c.replace(&a, o))
                }
            } else {
                new_examples.push(c.clone());
            }
        }
        examples = new_examples;
    }

    // Run examples
    for c in &examples {
        println!();
        println!("RUNNING {c}");
        println!();
        if run_example(c).is_err() {
            outcomes.fails += 1;
        } else {
            outcomes.passes += 1;
        }
    }
    outcomes
}

fn main() -> ExitCode {
    let dir = cargo_toml::find();
    let outcomes = run_all_examples(&dir, None);

    println!();
    println!("SUMMARY");
    if outcomes.passes + outcomes.fails == 0 {
        println!("Couldn't find any examples to run.");
        ExitCode::FAILURE
    } else {
        println!("{} examples ran successfully.", outcomes.passes);
        if outcomes.fails == 0 {
            ExitCode::SUCCESS
        } else {
            println!("{} examples encountered errors.", outcomes.fails);
            ExitCode::FAILURE
        }
    }
}
