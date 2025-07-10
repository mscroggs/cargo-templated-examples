//! cargo-template-examples
//!
//! Install using `cargo install cargo-templated-examples`
//!
//! Run using: `cargo templated-examples`
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

use cargo_toml::Manifest;
use std::{
    collections::HashMap,
    env, fs,
    path::Path,
    process::{Command, ExitCode},
};

/// Load template arguments from the package.metadata.templated-examples section of Cargo.toml
fn load_cargo_toml_args(args: &mut HashMap<String, Vec<String>>) {
    let cargo_toml =
        Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml"))
            .expect("Could not parse Cargo.toml");

    if let Some(p) = cargo_toml.package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("templated-examples")
    {
        for (i, j) in e
            .as_table()
            .expect("Could not parse package.metadata.templated-examples")
        {
            args.insert(
                i.clone(),
                j.as_array()
                    .expect("Values in package.metadata.templated-examples must be arrays of strings")
                    .iter()
                    .map(|value| String::from(value.as_str()
                    .expect("Values in package.metadata.templated-examples must be arrays of strings")))
                    .collect::<Vec<_>>(),
            );
        }
    }
}

/// Load template arguments input via the command line
fn load_command_line_args(args: &mut HashMap<String, Vec<String>>) {
    let input_args = env::args().collect::<Vec<_>>();
    assert_eq!(input_args[1], "templated-examples");
    for i in 1..input_args.len() / 2 {
        args.insert(
            input_args[2 * i].clone(),
            input_args[2 * i + 1]
                .split(",")
                .map(String::from)
                .collect::<Vec<_>>(),
        );
    }
}

/// Get example command from a file
fn get_example_command(file: &Path) -> String {
    let file_stem = file
        .file_stem()
        .expect("Error parsing file name")
        .to_str()
        .expect("Error parsing file name");
    for line in fs::read_to_string(file)
        .expect("Error reading example file")
        .lines()
    {
        if let Some(c) = line.strip_prefix("//? ") {
            return format!("cargo {c} --example {file_stem} --release");
        }
    }
    format!("run --example {file_stem} --release")
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
        Some(_) => Err("Example run failed"),
        None => Err("Example run failed"),
    }
}

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    // Load all template examples from files
    let mut examples = vec![];
    for file in fs::read_dir("examples").expect("Could not find examples directory") {
        let file = file.expect("Error reading examples directory").path();
        if let Some(e) = file.extension()
            && e == "rs"
        {
            examples.push(get_example_command(&file));
        }
    }

    // Substitute all template arguments
    let mut template_args = HashMap::new();
    load_cargo_toml_args(&mut template_args);
    load_command_line_args(&mut template_args);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_example_command() {
        let file = Path::new("example-crate/examples/parallel.rs");
        let command = get_example_command(file);
        assert_eq!(command, "cargo mpirun -n {{NPROCESSES}} --example parallel --release");
    }
}
