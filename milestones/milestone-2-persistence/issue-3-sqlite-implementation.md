# Implement SQLite Repository Adapter

**Milestone**: 2 - Persistence Layer

## Why

This is the "adapter" that implements your repository port. You'll learn how `sqlx` provides compile-time checked queries - the `query!` and `query_as!` macros verify your SQL against the actual database schema at compile time. This catches typos and schema mismatches that would be runtime errors in most other languages.

You'll also work with connection pools (`SqlitePool`), which manage database connections efficiently for async operations.

## What

**Goals:**
- Create `src/adapters/sqlite_repository.rs`
- Define a `SqliteMantraRepository` struct containing a `SqlitePool`
- Implement the `MantraRepository` trait for `SqliteMantraRepository`
- Use `sqlx::query!` macro for compile-time verified queries
- Write SQL queries for:
  - Fetching all mantras
  - Fetching history within N days
  - Inserting sent records
  - Adding new mantras
- Handle SQLite-specific errors and map them to your repository errors
- Create a constructor that takes a database URL and returns a pool
- Set up offline mode for sqlx: `cargo sqlx prepare`

**Acceptance Criteria:**
- [ ] `SqliteMantraRepository` implements `MantraRepository` trait
- [ ] All trait methods have working SQL implementations
- [ ] Uses `sqlx::query!` and `query_as!` macros for type safety
- [ ] Proper error handling and conversion
- [ ] Connection pool properly initialized
- [ ] Can build project without a running database (offline mode)

## Resources

- [SQLx Query Macros](https://docs.rs/sqlx/latest/sqlx/macro.query.html)
- [SQLx Offline Mode](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query)
- [SqlitePool Documentation](https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqlitePool.html)
- [Error Handling Patterns](https://nick.groenen.me/posts/rust-error-handling/)
- [Working with DateTime in SQLite](https://docs.rs/sqlx/latest/sqlx/types/index.html)
