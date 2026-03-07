# Build CLI with Clap

**Milestone**: 4 - CLI & Integration

## Why

`clap` (Command Line Argument Parser) is the most popular CLI framework in Rust. Its derive API uses Rust's powerful macro system to generate CLI parsing code at compile time. This is very different from Go's `flag` package or C#'s command line parsers - it's more declarative and catches errors at compile time.

You'll learn about Rust's derive macros, attribute macros, and how to structure a CLI application with multiple subcommands.

## What

**Goals:**
- Use `clap` with derive feature in `Cargo.toml`
- Create `src/cli/mod.rs` with command definitions
- Implement subcommands:
  - `send` - Send today's mantra
  - `add` - Add a new mantra to the database
  - `list` - List all mantras
  - `history` - Show sent history
- Use clap's derive attributes: `#[command]`, `#[arg]`
- Add help text and descriptions for each command
- Parse arguments and validate input
- Return structured command enums

**Acceptance Criteria:**
- [ ] CLI has multiple subcommands (send, add, list, history)
- [ ] Each command has helpful descriptions
- [ ] `--help` works for the app and each subcommand
- [ ] Arguments are properly typed (no stringly-typed APIs)
- [ ] CLI parsing code is generated via derive macros
- [ ] Proper error messages for invalid input

## Resources

- [Clap Documentation](https://docs.rs/clap/latest/clap/)
- [Clap Derive Tutorial](https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/README.md)
- [Rust CLI Book](https://rust-cli.github.io/book/index.html)
- [Command Line Interface Guidelines](https://clig.dev/)
