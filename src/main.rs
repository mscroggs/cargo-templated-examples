//! cargo-templated-examples

use std::process::ExitCode;
use std::env;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

fn main() -> ExitCode {
    println!("Hello, world!");

    let cargo_toml = cargo_toml::Manifest::from_str(&fs::read_to_string("Cargo.toml").expect("Cannot read Cargo.toml")).unwrap();

    let template_args = if let Some(p) = cargo_toml.package && let Some(m) = p.metadata && let Some(e) = m.get("templated_examples") {
        e.as_table().unwrap().iter().map(|(i, j)| (i.clone(), j.clone())).collect::<HashMap<_, _>>()
/*
        let mut a = HashMap::new()
        for (key, value) in e.as_table().unwrap() {
            a.insert(key.clone(), value.clone());
        }
        a
*/
    } else {
        HashMap::new()
    };

    println!("{template_args:?}");
    println!("{:?}", template_args.get("NPROCESSES").unwrap()[1]);

    //
    let mut shell = if cfg!(target_os = "windows") {
        let mut shell = Command::new("cmd");
        shell.arg("/C");
        shell
    } else {
        let mut shell = Command::new("sh");
        shell.arg("-c");
        shell
    };

    ExitCode::FAILURE
}
