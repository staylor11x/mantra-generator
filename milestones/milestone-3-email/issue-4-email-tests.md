# Add Email Service Tests

**Milestone**: 3 - Email Delivery

## Why

Testing email sending is tricky - you don't want to send real emails during tests. You'll learn about test doubles in Rust: creating mock implementations of traits. You might also use a local SMTP server like `mailhog` for integration testing.

This introduces the concept of dependency injection in Rust and how trait objects (`Box<dyn EmailService>`) enable runtime polymorphism.

## What

**Goals:**
- Create a `MockEmailService` that implements `EmailService` trait
- Mock should record sent emails rather than actually sending them
- Write unit tests using the mock:
  - Verify email content is formatted correctly
  - Check that proper recipient is used
  - Validate subject line formatting
- Optional: Set up `mailhog` Docker container for integration tests
- Use `Arc<Mutex<Vec<SentEmail>>>` or similar to track sent emails in mock
- Test error handling scenarios

**Acceptance Criteria:**
- [ ] `MockEmailService` implements `EmailService` trait
- [ ] Mock records sent emails for verification
- [ ] Unit tests verify email formatting and content
- [ ] Tests check error handling paths
- [ ] All tests pass without sending actual emails
- [ ] Optional: Integration test with mailhog

## Resources

- [Trait Objects in Rust](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Arc and Mutex for Shared State](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Mailhog - Email Testing](https://github.com/mailhog/MailHog)
- [Test Doubles](https://martinfowler.com/bliki/TestDouble.html)
- [Mocking in Rust](https://asomers.github.io/mock_shootout/)
