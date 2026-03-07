# Add Integration Tests for Persistence

**Milestone**: 2 - Persistence Layer

## Why

Integration tests verify that your code works with real external systems (in this case, SQLite). Unlike unit tests, these tests use an actual database. You'll learn about test fixtures, test isolation (each test should use a fresh database), and async testing with `tokio::test`.

This is also where you'll appreciate Rust's fearless concurrency - you can run tests in parallel safely.

## What

**Goals:**
- Create `tests/repository_tests.rs` (integration test in `tests/` directory)
- Write helper function to create a temporary test database
- Implement tests for:
  - Adding and retrieving mantras
  - Recording sent history
  - Querying history with date ranges
  - Ensuring sent records persist correctly
- Use `#[tokio::test]` attribute for async tests
- Clean up test databases after tests (use RAII with Drop trait or temp directories)
- Test error cases (e.g., database connection failures)

**Acceptance Criteria:**
- [ ] Integration tests in `tests/` directory
- [ ] Each test uses an isolated database instance
- [ ] Tests use `#[tokio::test]` for async execution
- [ ] Tests verify actual database operations
- [ ] All integration tests pass
- [ ] Proper cleanup of test databases

## Resources

- [The Rust Book - Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)
- [tokio::test macro](https://docs.rs/tokio/latest/tokio/attr.test.html)
- [tempfile crate](https://docs.rs/tempfile/latest/tempfile/) - for temporary test databases
- [Testing async Rust](https://rust-lang.github.io/async-book/09_example/index.html)
- [RAII in Rust](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
