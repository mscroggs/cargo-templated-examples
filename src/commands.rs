//! Cargo commands

use crate::parsing::parse_string_if_quoted;

/// A build type
#[derive(Clone, Debug, PartialEq)]
pub enum BuildType {
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
    pub fn from_str(s: &str) -> BuildType {
        match s {
            "debug" => BuildType::Debug,
            "release" => BuildType::Release,
            p => BuildType::Profile(p.to_string()),
        }
    }
}

/// A command to be run
#[derive(Clone, Debug, PartialEq)]
pub struct CargoCommand {
    run: String,
    example_name: String,
    args: Vec<(String, String)>,
    features: Vec<String>,
    build: BuildType,
}

impl CargoCommand {
    /// Create new
    pub fn new(example_name: String) -> CargoCommand {
        CargoCommand {
            run: String::from("run"),
            example_name,
            args: vec![],
            features: vec![],
            build: BuildType::Default,
        }
    }

    /// Convert command to string
    pub fn as_string(&self) -> String {
        let mut c = format!("cargo {}", self.run);
        for (key, value) in &self.args {
            c.push_str(&format!(" {key} {value}"));
        }
        c.push_str(&format!(" --example {}", self.example_name));
        if !self.features.is_empty() {
            c.push_str(&format!(" --features \"{}\"", self.features.join(",")));
        }
        match &self.build {
            BuildType::Debug => {}
            BuildType::Release => {
                c.push_str(" --release");
            }
            BuildType::Profile(p) => {
                c.push_str(&format!(" --profile {p}"));
            }
            BuildType::Default => {
                panic!("Cannot use default as run mode.");
            }
        }
        c
    }

    /// Set default build type
    pub fn set_default_build_type(&mut self, build: &BuildType) {
        if *build == BuildType::Default {
            panic!("Cannot set default build type to BuildType::Default");
        }
        if self.build == BuildType::Default {
            self.build = build.clone();
        }
    }

    /// Set build type, or panic if it has already been set to another non-default value
    pub fn set_build_type(&mut self, build: &BuildType) {
        self.set_default_build_type(build);
        if self.build != *build {
            panic!(
                "Inconsistent build types set for example \"{}\"",
                self.example_name
            );
        }
    }

    /// Set required features
    pub fn set_required_features(&mut self, features: &[String]) {
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
    pub fn from_str(c: &str, example_name: &str) -> CargoCommand {
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
        let example_name = String::from(example_name);
        CargoCommand {
            run,
            example_name,
            args,
            features,
            build,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
