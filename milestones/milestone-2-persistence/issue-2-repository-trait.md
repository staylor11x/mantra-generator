# Implement Repository Trait (Port)

**Milestone**: 2 - Persistence Layer

## Why

This implements the "ports" in hexagonal architecture (ports and adapters). A trait in Rust is like an interface in Go or C# - it defines behavior without implementation. By coding against a trait, your domain logic stays independent of SQLite - you could swap it for PostgreSQL or an in-memory implementation later.

Traits also enable Rust's powerful generics system and are how Rust does polymorphism without inheritance. Async traits (using `async-trait` crate) let you define asynchronous operations in the interface.

## What

**Goals:**
- Create `src/ports/repository.rs`
- Define a `MantraRepository` trait with async methods:
  - `async fn get_all_mantras(&self) -> Result<Vec<Mantra>>`
  - `async fn get_sent_history(&self, days: i32) -> Result<Vec<MantraId>>`
  - `async fn record_sent(&self, mantra_id: MantraId) -> Result<()>`
  - `async fn add_mantra(&self, text: String, category: Option<String>) -> Result<Mantra>`
- Use the `async-trait` crate for async trait methods
- Define repository-specific error types
- Keep trait database-agnostic (no SQL in the trait definition)
- Add trait bounds and lifetime annotations as needed

**Acceptance Criteria:**
- [ ] `MantraRepository` trait defined in `ports` module
- [ ] Trait methods are async (use `#[async_trait]`)
- [ ] Return types use `Result<T, E>` for error handling
- [ ] Methods take `&self` (shared reference)
- [ ] Error type defined for repository operations

## Resources

- [The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [async-trait crate](https://docs.rs/async-trait/latest/async_trait/)
- [Rust async/await](https://rust-lang.github.io/async-book/)
- [Repository Pattern](https://martinfowler.com/eaaCatalog/repository.html)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
