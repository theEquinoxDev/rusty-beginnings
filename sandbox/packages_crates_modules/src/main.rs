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

   If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.


## Modules & use
- Modules let you organize code within a crate into groups for readability and maintainability.
- Modules also control the scope and privacy of paths, allowing you to restrict access to certain parts of your code.
- You can define modules using the `mod` keyword, and you can nest  modules within other modules.
- You can use the `use` keyword to bring paths into scope, making it easier to reference items defined in other modules.
- You can define modules in the same file or in separate files, depending on your project's organization needs.
- The `pub` keyword is used to make items public, allowing them to be accessed from outside their module.

## Paths
- Paths are a way of naming an item, such as a struct, function, or module.
- Paths can be absolute or relative.
- An absolute path starts from a crate root and uses the crate name as the first component.
- A relative path starts from the current module and uses `self`, `super`, or an  identifier in the current module.
- You can use paths to reference items defined in other modules, making it easier to organize and access your code.

