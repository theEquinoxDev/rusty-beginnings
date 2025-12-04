Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

## Packages & Crates
- A crate is the smallest amount of code that the Rust compiler considers at a time. 
- Crates can contain modules, and the modules may be defined in other files that get compiled with the crate
A crate can come in two parts 
  - A library crate that defines functionality intended to be shared with multiple projects. eg - rand crate which provides random number generation
  - A binary crate that defines a program that can be run. eg - `main.rs`

- A package is a bundle of one or more crates that provides a set of functionality.
- A package contains a Cargo.toml file that describes how to build those crates.
- A package can contain multiple crates, but it must contain at least one library or binary crate
- A package can contain:
  - One library crate
  - One or more binary crates
  - Both a library crate and one or more binary crates