# Wire Components Together (Dependency Injection)

**Milestone**: 4 - CLI & Integration

## Why

This is where you compose your application. Rust doesn't have a built-in dependency injection framework like C#'s, but you don't need one - Rust's ownership system and trait objects provide all you need. You'll create an application context that holds your initialized services.

This teaches the builder pattern, managing shared state with `Arc` (atomic reference counting), and understanding when to use trait objects vs concrete types.

## What

**Goals:**
- Create `src/app/context.rs` or similar
- Define an `AppContext` struct containing:
  - Repository (as trait object: `Arc<dyn MantraRepository>`)
  - Email service (as trait object: `Arc<dyn EmailService>`)
  - Configuration
- Implement builder pattern for `AppContext`
- Initialize all dependencies in `main()`:
  - Database connection pool
  - SMTP client
  - Configuration from environment
- Pass context to CLI command handlers
- Handle initialization errors gracefully
- Use `Arc` for shared ownership across async tasks

**Acceptance Criteria:**
- [ ] `AppContext` struct holds all dependencies
- [ ] Builder pattern for constructing context
- [ ] Proper error handling during initialization
- [ ] Dependencies injected via trait objects
- [ ] `Arc` used for sharing context
- [ ] Clear separation between setup and execution

## Resources

- [Arc Documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Builder Pattern in Rust](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
- [Dependency Injection in Rust](https://oswalt.dev/2021/09/building-a-simple-dependency-injection-container-in-rust/)
