# Setup Cargo Project with Dependencies

**Milestone**: 1 - Core Domain & Project Foundation

## Why

Starting a Rust project properly is crucial for maintainability. Cargo is Rust's build system and package manager, similar to Go's `go mod` or C#'s NuGet. Understanding how to structure a Cargo project and manage dependencies sets the foundation for everything that follows.

In Rust, dependencies are called "crates" and are managed in `Cargo.toml`. The dependency resolution and feature system in Cargo is more explicit than Go's modules, giving you fine-grained control over what code gets compiled.

## What

**Goals:**
- Initialize or verify a Cargo project structure
- Add essential dependencies to `Cargo.toml`:
  - `tokio` (async runtime - you'll learn why Rust needs this vs Go's built-in goroutines)
  - `clap` (CLI parsing with derive macros)
  - `sqlx` (compile-time checked SQL queries)
  - `lettre` (email sending)
  - `tracing` (structured logging, more powerful than simple print statements)
  - `thiserror` (ergonomic error handling)
  - `serde` (serialization/deserialization)
- Understand the difference between dependencies and dev-dependencies
- Create a basic `main.rs` that prints "Hello, Mantra Generator!"
- Successfully run `cargo build` and `cargo run`

**Acceptance Criteria:**
- [ ] `Cargo.toml` contains all required dependencies with appropriate versions
- [ ] Project compiles without errors (`cargo build`)
- [ ] `cargo run` executes and prints a welcome message
- [ ] Project follows standard Cargo directory structure (`src/`, `Cargo.toml`, `Cargo.lock`)

## Resources

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cargo.toml Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Crates.io](https://crates.io) - The Rust package registry
- [Tokio Documentation](https://tokio.rs/tokio/tutorial) - Understanding async Rust
- [Rust Dependencies vs Dev-Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
