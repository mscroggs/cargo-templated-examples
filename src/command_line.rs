//! Functions to read command line input

use std::{collections::HashMap, env};

/// Special command line arguments
pub struct SpecialArgs {
    /// --features
    pub features: Vec<String>,
}

impl SpecialArgs {
    fn new() -> Self {
        Self { features: vec![] }
    }
}

/// Load template arguments input via the command line
pub fn load_args(args: &mut HashMap<String, Vec<String>>) {
    let input_args = env::args().collect::<Vec<_>>();
    assert_eq!(input_args[1], "templated-examples");
    for i in 1..input_args.len() / 2 {
        if input_args[2 * i] == "--features" {
            continue;
        }
        args.insert(
            input_args[2 * i].clone(),
            input_args[2 * i + 1]
                .split(",")
                .map(String::from)
                .collect::<Vec<_>>(),
        );
    }
}

/// Load special arguments input via the command line
pub fn load_special_args() -> SpecialArgs {
    let mut args = SpecialArgs::new();
    let input_args = env::args().collect::<Vec<_>>();
    assert_eq!(input_args[1], "templated-examples");
    for i in 1..input_args.len() / 2 {
        if input_args[2 * i] == "--features" {
            args.features = input_args[2 * i + 1]
                .split(",")
                .map(String::from)
                .collect::<Vec<_>>();
        }
    }
    args
}
