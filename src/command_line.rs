//! Functions to read command line input

use std::{collections::HashMap, env};

/// Load template arguments input via the command line
pub fn load_args(args: &mut HashMap<String, Vec<String>>) {
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
