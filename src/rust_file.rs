//! Functions to read information from a rust file

use crate::CargoCommand;
use std::{fs, path::Path};

/// Load command from file (line starting //?)
pub fn load_command(eg: &str) -> Option<CargoCommand> {
    for line in fs::read_to_string(Path::new(&format!("examples/{eg}.rs")))
        .expect("Error reading example file")
        .lines()
    {
        if let Some(c) = line.strip_prefix("//? ") {
            return Some(CargoCommand::from_str(c, eg));
        }
    }
    None
}
