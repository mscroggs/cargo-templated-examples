# cargo-templated-examples
cargo-templated-examples is a cargo extension that allows you to run all examples in
your crate with custom templated run command.

## Installing
cargo-templated-examples can be installed by running:

```bash
cargo install cargo-templated-examples
```

## Usage
cargo-templated-examples can be run using:

```bash
cargo templated-examples
```

### Custom commands
A custom run command for an example can be set by either adding a line starting with `//?` to the
example file or by adding metadata in the Cargo.toml file.

To set the custom command in the example file, the cargo command should be written on a line
starting with `//?`. For example, the line
```//? mpirun```
to the file intro_demo.rs would lead to the example being run using the command
`cargo mpirun --example intro_demo` (with release mode used by default).

To set a command in the Cargo.toml file, a section called `[package.metadata.example.<EXAMPLE_NAME>.templated-examples]`
should be added. This section can include values for `command` and/or `build`: `command` will set
the cargo command to use and `build` will set the build type. For example, adding
```toml
[package.metadata.example.intro_demo.templated-examples]
command = "mpirun"
build = "release"
```
would lead to the command `cargo mpirun --example intro_demo --release` being run.

If commands are set in both places and do not match, then cargo-templated-example will panic.

### Templating
A template variable can be included in a run command by including the variable name
between pairs of curly braces. For example, the line
```rust
//? mpirun -n {{NPROCESSES}}
```
includes the template variable `NPROCESSES`.

### Passing template values
The values that template variables take can be passed in either via a crate's Cargo.toml file
or via the command line.

Values can be passed in via a crate's Cargo.toml file by adding a
`package.metadata.templated-examples` section. For example, adding
```toml
[package.metadata.templated-examples]
NPROCESSES = ["2", "4"]
```
would lead to the variable `NPROCESSES` taking the values `2` and `4`: this would lead to any
example whose command contains this variable being run twice (once with each value).

Valued can be passed via the command line by writing variable names and comma-separated list
of values after `cargo templated-examples`. For example, the command
```bash
cargo templated-example NPROCESSES 1,5
```
would lead to the variable `NPROCESSES` taking the values `1` and `5`.

If values are passed in both ways, thoese passed via the command line will be used.

### Build type
The build type (debug or release) can be set by setting a value for `build` option in the
`package.metadata.templated-examples` section of Cargo.toml. For example, adding
```toml
[package.metadata.templated-examples]
build = "debug"
```
will set the default build type to debug. If this value is not set, the default build type
will be release.

### Example
An example of the usage of cargo-templated-example can be found in the 
[example-crate](https://github.com/mscroggs/cargo-templated-examples/tree/main/example-crate)
folder of the GitHub repository.
