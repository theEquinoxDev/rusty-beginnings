🌟 Overview

Welcome to `rusty-beginnings` — my learning playground for Rust. This repository collects small projects, experiments, notes, and practical exercises that help me learn Rust concepts through hands-on code. The aim is to capture both the successes and the mistakes so I and others can learn from them.

**Repository Goals**
- **Learn by building:** create bite-sized projects to solidify language concepts.
- **Document the journey:** write clear notes that explain why code works (and why it doesn’t).
- **Explore tooling:** use Cargo, common crates, and Rust tooling workflows.
- **Prepare for real projects:** graduate from examples to small CLI tools and services.

**Who this repo is for**
- Me (the author) — to track progress.
- Developers learning Rust who want concrete, annotated examples.
- Anyone curious about how typical beginner mistakes get resolved in Rust.

**Repository Structure (high level)**
```
rusty-beginnings/
│
├── Docs/            # Project directories, notes, and experiments
│   ├── projects/    # Longer exercises and organised projects
│   └── sandbox/     # Small quick experiments and playgrounds
│       ├── hello_world/
│       └── hello_cargo/
│
└── README.md        # This file
```

**Quick orientation**
- `Docs/sandbox/` contains tiny projects meant for experimenting quickly.
- `Docs/projects/` is for longer-term exercises that may grow over time.

**Example projects included**
- `Docs/sandbox/hello_world` — minimal `cargo` hello-world example.
- `Docs/sandbox/hello_cargo` — demonstrates a small Cargo-based binary with simple build artifacts.

**Why these examples matter**
They illustrate core concepts (project layout, `Cargo.toml`, `src/main.rs`) and the Cargo build lifecycle (`target/` artifacts). These are intentionally small so you can read, run, and modify them quickly.

**How to Run and Build**
All example projects use Cargo. From the repository root you can run any example by changing into its directory and using standard Cargo commands.

Windows `cmd.exe` commands:
```
cd Docs\sandbox\hello_world
cargo run

cd ..\hello_cargo
cargo build
cargo run --release
```

Notes:
- `cargo run` builds and runs the example.
- `cargo build` produces artifacts in the `target/` directory.
- Use `--release` for an optimized build.

**Learning Focus Areas**
- Ownership & borrowing: why the compiler rejects some code and how to fix it.
- Error handling with `Result` and `Option`.
- Modules and simple project layout.
- Using `Cargo.toml` to manage dependencies.
- Debug vs release builds and reading compiler errors.

**Conventions & Tips**
- Keep examples small and focused: one concept per example.
- Add comments and notes near code showing what you learned.
- Commit frequently with descriptive messages: explains why a change was made.

**Roadmap (near-term)**
- Add annotated examples for borrowing and ownership.
- Create a small CLI utility demonstrating argument parsing.
- Add basic tests and show how to run them with `cargo test`.

**Long-term plans**
- Build small async services (with `tokio`).
- Integrate `serde` for serialization demos.
- Create a documented walkthrough of lifetimes.

**Contributing / Personal workflow**
- This repo is primarily a personal learning log — pull requests are welcome if you want to add educational examples or corrections.
- When making changes, include short notes in the commit message about the learning point.

**Useful commands**
```
cd Docs\sandbox\hello_world
cargo run                 # build and run
cargo build               # build artifacts into target/
cargo test                # run tests (when added)

cd ..\..\projects\<project-name>
cargo run
```

**Learning resources**
- The official Rust book: https://doc.rust-lang.org/book/
