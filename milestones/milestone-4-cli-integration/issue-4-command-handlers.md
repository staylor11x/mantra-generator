# Implement Command Handlers and Main Loop

**Milestone**: 4 - CLI & Integration

## Why

This brings everything together - the command handlers execute your business logic using the composed dependencies. You'll wire up the CLI commands to actually perform database operations, select mantras, and send emails.

This is where you'll see the benefits of your architecture: clean, testable code where each layer has a single responsibility.

## What

**Goals:**
- Implement handler for `send` command:
  - Load all mantras from database
  - Load sent history
  - Use domain logic to select a mantra
  - Send via email service
  - Record in sent history
- Implement handler for `add` command:
  - Parse input
  - Add to database via repository
- Implement handler for `list` command:
  - Fetch and display all mantras
- Implement handler for `history` command:
  - Fetch and display sent history with dates
- Add proper error handling and user-friendly error messages
- Make `main()` async with `#[tokio::main]`
- Handle shutdown gracefully

**Acceptance Criteria:**
- [ ] All CLI commands have working implementations
- [ ] `send` command successfully sends a daily mantra
- [ ] Database operations work correctly
- [ ] User-friendly error messages
- [ ] Graceful error handling throughout
- [ ] Can run end-to-end: `cargo run -- send`

## Resources

- [tokio::main macro](https://docs.rs/tokio/latest/tokio/attr.main.html)
- [Error Handling Patterns](https://nick.groenen.me/posts/rust-error-handling/)
- [anyhow crate](https://docs.rs/anyhow/latest/anyhow/) - for application error handling
- [Working with async in main](https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html)
