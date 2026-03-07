# Implement Email Service Trait (Port)

**Milestone**: 3 - Email Delivery

## Why

Similar to the repository pattern, defining an email service trait keeps your delivery mechanism pluggable. This is good architecture - you might want to send via SendGrid API, AWS SES, or local SMTP. The trait abstraction means your core logic doesn't care about the implementation.

This also makes testing easier - you can create a mock email service that doesn't actually send emails during tests.

## What

**Goals:**
- Create `src/ports/email_service.rs`
- Define an `EmailService` trait with async methods:
  - `async fn send_mantra(&self, recipient: &str, mantra: &Mantra) -> Result<()>`
- Consider additional methods like:
  - `async fn test_connection(&self) -> Result<()>` (verify SMTP works)
- Define email-specific error types
- Use `#[async_trait]` for async trait methods
- Keep trait implementation-agnostic (no SMTP details in the trait)

**Acceptance Criteria:**
- [ ] `EmailService` trait defined in `ports` module
- [ ] Trait methods are async
- [ ] Clear method signatures for sending mantras
- [ ] Email-specific error type defined
- [ ] Trait is decoupled from any specific email provider

## Resources

- [The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [async-trait crate](https://docs.rs/async-trait/latest/async_trait/)
- [Error Handling Best Practices](https://nick.groenen.me/posts/rust-error-handling/)
- [Email Standards - RFC 5322](https://datatracker.ietf.org/doc/html/rfc5322)
