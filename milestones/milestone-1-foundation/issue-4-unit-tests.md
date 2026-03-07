# Write Unit Tests for Domain Logic

**Milestone**: 1 - Core Domain & Project Foundation

## Why

Rust has first-class testing support built into Cargo. Unlike Go where tests are in `_test.go` files, Rust encourages writing tests alongside your code in the same file (for unit tests) or in a `tests/` directory (for integration tests).

Testing pure domain logic is straightforward because there are no external dependencies to mock. This introduces you to Rust's testing annotations, assertion macros, and the `#[cfg(test)]` attribute.

## What

**Goals:**
- Add a `#[cfg(test)]` module at the bottom of `selector.rs`
- Write test cases for:
  - Selecting a mantra when history is empty
  - Selecting a mantra when some have been used
  - Error case when all mantras are in history
  - Error case when no mantras are available
  - Edge case with a single mantra
- Use Rust's assertion macros: `assert!`, `assert_eq!`, `assert_matches!`
- Learn the `#[should_panic]` attribute for testing error paths
- Run tests with `cargo test`
- Understand test organization: `#[test]` attribute marks test functions

**Acceptance Criteria:**
- [ ] At least 5 test cases covering happy path and error cases
- [ ] All tests pass (`cargo test`)
- [ ] Tests use descriptive names (e.g., `test_selection_excludes_history`)
- [ ] Tests are isolated (each test is independent)
- [ ] Test coverage includes edge cases

## Resources

- [The Rust Book - Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [Testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
- [assert_matches! macro](https://doc.rust-lang.org/std/macro.assert_matches.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
