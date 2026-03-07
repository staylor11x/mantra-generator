# Implement Email Service Adapter

**Milestone**: 3 - Email Delivery

## Why

Now you'll implement the `EmailService` trait using `lettre`. You'll construct email messages, format HTML content, and handle email-specific errors. This teaches working with the builder pattern extensively and composing complex objects in Rust.

You'll also learn about email message structure (From, To, Subject, Body) and the difference between plain text and HTML emails.

## What

**Goals:**
- Create `src/adapters/smtp_email_service.rs`
- Implement `EmailService` trait for an `SmtpEmailService` struct
- Format mantra as an attractive email:
  - Subject: "Your Daily Mantra - [date]"
  - Both plain text and HTML versions
  - Include mantra text, category (if present)
- Use `lettre`'s message builder (`Message::builder()`)
- Handle multipart messages (text + HTML alternative)
- Provide constructor that accepts SMTP configuration
- Map SMTP errors to your custom error type
- Consider adding retry logic for transient failures

**Acceptance Criteria:**
- [ ] `SmtpEmailService` implements `EmailService` trait
- [ ] Emails formatted with both plain text and HTML
- [ ] Professional email formatting with proper headers
- [ ] Error handling for SMTP failures
- [ ] Successfully sends test emails
- [ ] Proper async implementation

## Resources

- [Lettre Message Builder](https://docs.rs/lettre/latest/lettre/message/struct.Message.html)
- [MIME Multipart Messages](https://docs.rs/lettre/latest/lettre/message/struct.MultiPart.html)
- [HTML Email Best Practices](https://www.campaignmonitor.com/dev-resources/guides/coding/)
- [Email Header Standards](https://datatracker.ietf.org/doc/html/rfc5322#section-3.6)
