//! Functions to read information from Cargo.toml

use crate::{BuildType, CargoCommand};
use cargo_toml::Manifest;
use std::{collections::HashMap, fs};

// Load Cargo.toml
fn cargo_toml() -> Manifest {
    Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml"))
        .expect("Could not parse Cargo.toml")
}

/// Load template arguments from the package.metadata.templated-examples section of Cargo.toml
pub fn load_args(args: &mut HashMap<String, Vec<String>>) {
    if let Some(p) = cargo_toml().package
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
pub fn get_default_build() -> BuildType {
    if let Some(p) = cargo_toml().package
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

/// Load required features for an example
pub fn load_required_features(eg: &str) -> Vec<String> {
    for e in cargo_toml().example {
        if Some(eg) == e.name.as_deref() {
            return e.required_features;
        }
    }
    vec![]
}

/// Load command from Cargo.toml section [package.metedata.example.{{eg}}.templated-examples]
pub fn load_command(eg: &str) -> Option<CargoCommand> {
    if let Some(p) = cargo_toml().package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("example")
        && let Some(ex) = e.get(eg)
        && let Some(d) = ex.get("templated-examples")
        && let Some(c) = d.get("command")
    {
        let mut cmd = CargoCommand::from_str(c.as_str().expect("Command must be a string"), eg);
        if let Some(b) = load_build(eg) {
            cmd.set_build_type(&b);
        }
        Some(cmd)
    } else {
        None
    }
}

/// Load build from Cargo.toml section [package.metedata.example.{{eg}}.templated-examples]
pub fn load_build(eg: &str) -> Option<BuildType> {
    if let Some(p) = cargo_toml().package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("example")
        && let Some(ex) = e.get(eg)
        && let Some(d) = ex.get("templated-examples")
        && let Some(c) = d.get("build")
    {
        Some(BuildType::from_str(
            c.as_str().expect("Command must be a string"),
        ))
    } else {
        None
    }
}
