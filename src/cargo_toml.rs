//! Functions to read information from Cargo.toml

use crate::{BuildType, CargoCommand};
use cargo_toml::Manifest;
use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

/// Join a directory and a file name
pub fn join(part1: &impl AsRef<Path>, part2: &str) -> PathBuf {
    let mut out = PathBuf::from(part1.as_ref());
    out.push(part2);
    out
}

/// Find directory containing Cargo.toml
pub fn find() -> PathBuf {
    let mut dir = current_dir().expect("Cannot find current dir");
    while !join(&dir, "Cargo.toml").exists() {
        dir = dir.parent().expect("Cannot find Cargo.toml").to_path_buf();
    }
    dir
}

/// Load Cargo.toml
fn cargo_toml(dir: &impl AsRef<Path>) -> Manifest {
    Manifest::from_str(
        &fs::read_to_string(join(dir, "Cargo.toml")).expect("Cannot read Cargo.toml"),
    )
    .expect("Could not parse Cargo.toml")
}

/// Load template arguments from the package.metadata.templated-examples section of Cargo.toml
pub fn load_args(dir: &impl AsRef<Path>, args: &mut HashMap<String, Vec<String>>) {
    if let Some(p) = cargo_toml(dir).package
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
pub fn get_default_build(dir: &impl AsRef<Path>) -> BuildType {
    if let Some(p) = cargo_toml(dir).package
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

/// Get default build type
pub fn get_workspace(dir: &impl AsRef<Path>) -> Option<Vec<String>> {
    if let Some(w) = cargo_toml(dir).workspace {
        return Some(w.members);
    }
    None
}

/// Load required features for an example
pub fn load_required_features(dir: &impl AsRef<Path>, eg: &str) -> Vec<String> {
    for e in cargo_toml(dir).example {
        if Some(eg) == e.name.as_deref() {
            return e.required_features;
        }
    }
    vec![]
}

/// Load available features for a crate
pub fn load_available_features(dir: &impl AsRef<Path>) -> Vec<String> {
    cargo_toml(dir).features.iter().map(|i| i.0.clone()).collect::<Vec<_>>()
}

/// Load command from Cargo.toml section [package.metedata.example.{{eg}}.templated-examples]
pub fn load_command(dir: &impl AsRef<Path>, eg: &str) -> Option<CargoCommand> {
    if let Some(p) = cargo_toml(dir).package
        && let Some(m) = p.metadata
        && let Some(e) = m.get("example")
        && let Some(ex) = e.get(eg)
        && let Some(d) = ex.get("templated-examples")
    {
        let mut cmd = if let Some(c) = d.get("command") {
            CargoCommand::from_str(
                c.as_str()
                    .expect("Command must be a string for example \"{eg}\""),
                eg,
            )
        } else {
            CargoCommand::new(String::from(eg))
        };
        if let Some(b) = d.get("build") {
            cmd.set_build_type(&BuildType::from_str(
                b.as_str()
                    .expect("Command must be a string for example \"{eg}\""),
            ));
        }
        Some(cmd)
    } else {
        None
    }
}
