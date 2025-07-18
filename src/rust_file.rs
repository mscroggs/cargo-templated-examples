//! Functions to read information from a rust file

use crate::CargoCommand;
use crate::cargo_toml::join;
use std::{fs, path::Path};

/// Load command from file (line starting //?)
pub fn load_command(dir: &impl AsRef<Path>, eg: &str) -> Option<CargoCommand> {
    for line in fs::read_to_string(join(&join(dir, "examples"), &format!("{eg}.rs")))
        .expect("Error reading example file")
        .lines()
    {
        if let Some(c) = line.strip_prefix("//? ") {
            return Some(CargoCommand::from_str(c, eg));
        }
    }
    None
}
