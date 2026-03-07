# Implement Mantra Selection Logic

**Milestone**: 1 - Core Domain & Project Foundation

## Why

This is where you implement pure business logic - a function that selects a mantra based on history. In functional programming terms, this is a pure function: given the same inputs, it always returns the same output with no side effects.

This teaches key Rust concepts: working with collections (Vec, HashSet), iterators (way more powerful than Go's loops), the `Result<T, E>` type for error handling (similar to Go's error returns but more type-safe), and borrowing (passing data without transferring ownership).

## What

**Goals:**
- Create a `src/domain/selector.rs` module
- Implement a function with signature similar to:
  ```rust
  fn select_mantra(
      available: &[Mantra],
      history: &[MantraId],
  ) -> Result<&Mantra, SelectionError>
  ```
- Logic: select a random mantra from `available` that hasn't been sent recently (not in `history`)
- Define a custom `SelectionError` enum for error cases:
  - All mantras have been sent recently
  - No mantras available
- Use Rust's iterator methods (`.filter()`, `.collect()`)
- Use the `rand` crate for random selection
- Keep logic pure - no I/O, no side effects

**Acceptance Criteria:**
- [ ] Selection function returns a mantra not in history
- [ ] Returns error when all mantras exhausted
- [ ] Function signature uses references (borrowing, not ownership transfer)
- [ ] Custom error type defined with `thiserror`
- [ ] Logic uses iterator chains rather than manual loops

## Resources

- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [thiserror crate](https://docs.rs/thiserror/latest/thiserror/)
- [rand crate](https://docs.rs/rand/latest/rand/)
- [Understanding References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
