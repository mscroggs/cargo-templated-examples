//! cargo-template-examples
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
use cargo_toml::{get_default_build, load_args as cargo_toml_load_args};
use command_line::load_args as command_line_load_args;
use commands::{BuildType, CargoCommand, get_example_command};

use std::{
    collections::HashMap,
    fs,
    process::{Command, ExitCode},
};

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
        Some(_) => Err("Example run failed"),
        None => Err("Example run failed"),
    }
}

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    let default_build = get_default_build();

    // Load all template examples from files
    let mut examples = vec![];
    for file in fs::read_dir("examples").expect("Could not find examples directory") {
        let file = file.expect("Error reading examples directory").path();
        if let Some(e) = file.extension()
            && e == "rs"
        {
            let c = get_example_command(&file);
            examples.push(c.to_string(&default_build));
        }
    }

    // Substitute all template arguments
    let mut template_args = HashMap::new();
    cargo_toml_load_args(&mut template_args);
    command_line_load_args(&mut template_args);

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
            exit_code = ExitCode::FAILURE;
        }
    }
    exit_code
}
