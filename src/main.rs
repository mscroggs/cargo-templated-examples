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

/// A build type
#[derive(Clone, Debug, PartialEq)]
enum BuildType {
    /// Debug mode
    Debug,
    /// Release mode
    Release,
    /// Run using given profile
    Profile(String),
    /// Use default mode
    Default,
}

impl BuildType {
    /// Create from string
    fn from_str(s: &str) -> BuildType {
        match s {
            "debug" => BuildType::Debug,
            "release" => BuildType::Release,
            p => BuildType::Profile(p.to_string()),
        }
    }
}

/// A command to be run
#[derive(Clone, Debug, PartialEq)]
struct CargoCommand {
    run: String,
    example_name: String,
    args: Vec<(String, String)>,
    features: Vec<String>,
    build: BuildType,
}

/// If a string starts with a quote, parse what's inside the quotes
fn parse_string_if_quoted(s: &str) -> String {
    if s.starts_with("\"") || s.starts_with("'") {
        assert_eq!(s[0..1], s[s.len() - 1..s.len()]);
        let mut s = s[1..s.len() - 1].chars();
        let mut output = String::new();
        while let Some(c) = s.next() {
            if c == '\\' {
                output = format!(
                    "{output}{}",
                    s.next().expect("String cannot end with a backslash")
                );
            } else {
                output = format!("{output}{c}");
            }
        }
        output
    } else {
        s.to_string()
    }
}

impl CargoCommand {
    /// Convert command to string
    fn to_string(&self, default_build: &BuildType) -> String {
        let mut c = format!("cargo {}", self.run);
        for (key, value) in &self.args {
            c = format!("{c} {key} {value}");
        }
        c = format!("{c} --example {}", self.example_name);
        if !self.features.is_empty() {
            c = format!("{c} --features \"{}\"", self.features.join(","));
        }
        match match &self.build {
            BuildType::Default => default_build,
            x => x,
        } {
            BuildType::Debug => {}
            BuildType::Release => {
                c = format!("{c} --release");
            }
            BuildType::Profile(p) => {
                c = format!("{c} --profile {p}");
            }
            BuildType::Default => {
                panic!("Cannot use default as run mode.");
            }
        }
        c
    }

    /// Set default build type
    fn set_default_build_type(&mut self, build: &BuildType) {
        if *build == BuildType::Default {
            panic!("Cannot set default build type to BuildType::Default");
        }
        if self.build == BuildType::Default {
            self.build = build.clone();
        }
    }

    /// Set build type, or panic if it has already been set to another non-default value
    fn set_build_type(&mut self, build: &BuildType) {
        self.set_default_build_type(build);
        if self.build != *build {
            panic!(
                "Inconsistent build types set for example \"{}\"",
                self.example_name
            );
        }
    }

    /// Set required features
    fn set_required_features(&mut self, features: &[String]) {
        if self.features.is_empty() {
            self.features = features.to_vec();
        } else {
            for f in features {
                if !self.features.contains(f) {
                    panic!(
                        "Required feature \"{f}\" is missing from list of features in command for example \"{}\"",
                        self.example_name
                    );
                }
            }
        }
    }

    /// Create from a string
    fn from_str(c: &str, example_name: &str) -> CargoCommand {
        println!("{c}");
        let mut features = vec![];
        let mut args = vec![];
        let mut build = BuildType::Default;
        let mut c = c.split(" ");
        let run = String::from(c.next().expect("Command cannot be empty"));
        while let Some(i) = c.next() {
            match i {
                "--release" => {
                    if build != BuildType::Default {
                        panic!("Cannot set build type twice");
                    }
                    build = BuildType::Release;
                }
                "--profile" => {
                    if build != BuildType::Default {
                        panic!("Cannot set build type twice");
                    }
                    build = BuildType::Profile(String::from(
                        c.next().expect("Profile cannot be blank"),
                    ));
                }
                "--features" => {
                    // TODO: tidy this up
                    features = parse_string_if_quoted(c.next().expect("Features cannot be blank"))
                        .split(",")
                        .map(String::from)
                        .collect::<Vec<_>>();
                }
                _ => {
                    args.push((
                        String::from(i),
                        String::from(c.next().expect("Keys and values must come in pairs")),
                    ));
                }
            }
        }

        CargoCommand {
            run,
            example_name: String::from(example_name),
            args,
            features,
            build,
        }
    }
}

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
            if i != "build" {
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
}

/// Get default build type
fn get_default_build() -> BuildType {
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
            if i == "build" {
                return BuildType::from_str(j.as_str().expect("Build type must be a string"));
            }
        }
    }
    BuildType::Release
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

/// Get example command for a file
fn get_example_command(file: &Path) -> CargoCommand {
    let file_stem = file
        .file_stem()
        .expect("Error parsing file name")
        .to_str()
        .expect("Error parsing file name");

    let example_build = if let Some(p) =
        Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml"))
            .expect("Could not parse Cargo.toml")
            .package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("example")
        && let Some(ex) = e.get(file_stem)
        && let Some(d) = ex.get("templated-examples")
        && let Some(c) = d.get("build")
    {
        BuildType::from_str(c.as_str().expect("Build mode must be a string"))
    } else {
        BuildType::Default
    };

    let mut required_features = vec![];
    for e in Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml"))
        .expect("Could not parse Cargo.toml")
        .example
    {
        if Some(file_stem) == e.name.as_deref() {
            required_features = e.required_features;
            break;
        }
    }
    // Load command from file (line starting //?)
    let mut file_command = None;
    for line in fs::read_to_string(file)
        .expect("Error reading example file")
        .lines()
    {
        if let Some(c) = line.strip_prefix("//? ") {
            file_command = Some(CargoCommand::from_str(c, file_stem));
            break;
        }
    }

    // Load command from Cargo.toml [package.metedata.example.{{EXAMPLE}}.templated-examples]
    let cargo_toml_command = if let Some(p) =
        Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml"))
            .expect("Could not parse Cargo.toml")
            .package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("example")
        && let Some(ex) = e.get(file_stem)
        && let Some(d) = ex.get("templated-examples")
        && let Some(c) = d.get("command")
    {
        let c = c.as_str().expect("Command must be a string");
        Some(CargoCommand::from_str(c, file_stem))
    } else {
        None
    };

    // Return command
    if let Some(c) = file_command {
        if let Some(c2) = cargo_toml_command
            && c != c2
        {
            panic!("Commands set in file and Cargo.toml do not match");
        }
        c
    } else if let Some(c) = cargo_toml_command {
        c
    } else {
        CargoCommand {
            run: String::from("run"),
            example_name: String::from(file_stem),
            args: vec![],
            features: required_features,
            build: BuildType::Default,
        }
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
            examples.push(get_example_command(&file).to_string(&default_build));
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
        let command = get_example_command(file).to_string(&BuildType::Release);
        assert_eq!(
            command,
            "cargo mpirun -n {{NPROCESSES}} --example parallel --release"
        );
        let command = get_example_command(file).to_string(&BuildType::Debug);
        assert_eq!(command, "cargo mpirun -n {{NPROCESSES}} --example parallel");
    }

    #[test]
    fn test_from_str_features() {
        let c = CargoCommand::from_str("run --features \"one,two\"", "test");
        assert_eq!(c.features.len(), 2);

        let mut c = CargoCommand::from_str("run --features \"one,two\"", "test");
        c.set_required_features(&[String::from("one"), String::from("two")]);
        assert_eq!(c.features.len(), 2);

        let mut c = CargoCommand::from_str("run", "test");
        c.set_required_features(&[String::from("one"), String::from("two")]);
        assert_eq!(c.features.len(), 2);

        let mut c = CargoCommand::from_str("run --features \"one,two\"", "test");
        c.set_required_features(&[String::from("one")]);
        assert_eq!(c.features.len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_from_str_missing_feature() {
        let mut c = CargoCommand::from_str("run --features \"one\"", "test");
        c.set_required_features(&[String::from("one"), String::from("two")]);
    }

    #[test]
    fn test_from_str_build_type() {
        let c = CargoCommand::from_str("run --profile build", "test");
        assert_eq!(c.build, BuildType::Profile(String::from("build")));

        let mut c = CargoCommand::from_str("run", "test");
        c.set_default_build_type(&BuildType::Profile(String::from("build")));
        assert_eq!(c.build, BuildType::Profile(String::from("build")));

        let mut c = CargoCommand::from_str("run", "test");
        c.set_build_type(&BuildType::Profile(String::from("build")));
        assert_eq!(c.build, BuildType::Profile(String::from("build")));

        let mut c = CargoCommand::from_str("run --profile build", "test");
        c.set_default_build_type(&BuildType::Profile(String::from("build")));
        assert_eq!(c.build, BuildType::Profile(String::from("build")));

        let mut c = CargoCommand::from_str("run --profile build", "test");
        c.set_default_build_type(&BuildType::Debug);
        assert_eq!(c.build, BuildType::Profile(String::from("build")));
    }

    #[test]
    #[should_panic]
    fn test_from_str_incompatible_build_type() {
        let mut c = CargoCommand::from_str("run --profile build", "test");
        c.set_build_type(&BuildType::Debug);
        assert_eq!(c.build, BuildType::Profile(String::from("build")));
    }

    #[test]
    fn test_parse_if_quoted() {
        assert_eq!(parse_string_if_quoted("\"test\""), "test");
        assert_eq!(parse_string_if_quoted("\"test\\\\\""), "test\\");
        assert_eq!(parse_string_if_quoted("\"test\\\"\""), "test\"");
        assert_eq!(parse_string_if_quoted("test\\\""), "test\\\"");
    }
}
