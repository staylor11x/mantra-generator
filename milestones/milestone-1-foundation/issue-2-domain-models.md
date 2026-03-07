# Define Core Domain Models

**Milestone**: 1 - Core Domain & Project Foundation

## Why

Domain-driven design separates business logic from infrastructure concerns. In Rust, this means defining your core types independently of databases, APIs, or external services. This approach aligns with your architecture goal of supporting multiple delivery mechanisms (email now, HTTP API later).

Rust's type system is more powerful than Go's - you'll use enums, the `Option<T>` type (similar to nullable types in C#), and derive macros to automatically implement common traits. Understanding Rust's ownership model starts here with simple value types.

## What

**Goals:**
- Create a `src/domain/` module structure
- Define a `Mantra` struct containing:
  - `id` (use a newtype pattern for type safety, e.g., `MantraId`)
  - `text` (the actual mantra content)
  - `category` (optional - use `Option<String>`)
  - `created_at` (consider using `chrono` crate for timestamps)
- Define `MantraId` as a newtype wrapper around a primitive (e.g., `i64` or `Uuid`)
- Implement the `Display` trait for pretty-printing
- Add appropriate derives: `Debug`, `Clone`, `PartialEq`
- Learn the difference between `String` and `&str`

**Acceptance Criteria:**
- [ ] `src/domain/mantra.rs` defines `Mantra` and `MantraId`
- [ ] Models use idiomatic Rust types (`Option<T>`, owned vs borrowed)
- [ ] Structs derive common traits (`Debug`, `Clone`)
- [ ] Can create instances of `Mantra` in code
- [ ] Types are private by default, public API uses `pub` keyword appropriately

## Resources

- [The Rust Programming Language - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Newtype Pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
- [chrono crate](https://docs.rs/chrono/latest/chrono/)
- [Understanding String vs &str](https://blog.mgattozzi.dev/how-do-i-str-string/)
