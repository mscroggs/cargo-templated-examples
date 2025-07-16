//! Functions to read information from Cargo.toml

use std::{fs, path::Path};

/// Load command from file (line starting //?)
pub fn load_command(file: &Path) -> Option<String> {
    for line in fs::read_to_string(file)
        .expect("Error reading example file")
        .lines()
    {
        if let Some(c) = line.strip_prefix("//? ") {
            return Some(String::from(c));
        }
    }
    None
}
