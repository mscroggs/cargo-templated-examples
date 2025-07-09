//! cargo-template-examples
//!
//! Install using `cargo install cargo-templated-examples`
//!
//! Run using: `cargo templated-examples`
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

use cargo_toml::Manifest;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;

/// Load template arguments from the package.metadata.templated_examples section of Cargo.toml
fn load_args_from_cargo_toml() -> HashMap<String, Vec<String>> {
    let cargo_toml =
        Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml"))
            .unwrap();

    if let Some(p) = cargo_toml.package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("templated_examples")
    {
        e.as_table()
            .unwrap()
            .iter()
            .map(|(i, j)| {
                (
                    i.clone(),
                    j.as_array()
                        .unwrap()
                        .iter()
                        .map(|k| String::from(k.as_str().unwrap()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    } else {
        HashMap::new()
    }
}

/// Get example command from a file
fn get_example_command(file: &Path) -> String {
    let file_stem = file.file_stem().unwrap().to_str().unwrap();
    for line in fs::read_to_string(file).unwrap().lines() {
        if let Some(c) = line.strip_prefix("//? ") {
            return format!("{c} --example {file_stem}");
        }
    }
    format!("run --example {file_stem}")
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
    shell.arg(format!("cargo {example}"));

    let mut child = shell.spawn().unwrap();
    match child.wait().unwrap().code() {
        Some(0) => Ok(()),
        Some(_) => Err("Example run failed"),
        None => Err("Example run failed"),
    }
}

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    // Load all template examples from files
    let mut examples = vec![];
    for file in fs::read_dir("examples").unwrap() {
        let file = file.unwrap().path();
        if let Some(e) = file.extension()
            && e == "rs"
        {
            examples.push(get_example_command(&file));
        }
    }

    // Substitute all template arguments
    let template_args = load_args_from_cargo_toml();
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
    fn test_load_args() {
        assert_eq!(load_args_from_cargo_toml().len(), 0);
    }

    #[test]
    fn test_get_example_command() {
        let file = Path::new("example-crate/examples/parallel.rs");
        let command = get_example_command(file);
        assert_eq!(command, "mpirun -n {{NPROCESSES}} --example parallel");
    }
}
