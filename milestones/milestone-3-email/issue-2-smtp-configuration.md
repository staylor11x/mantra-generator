# Configure SMTP with Lettre

**Milestone**: 3 - Email Delivery

## Why

`lettre` is the standard email library for Rust. It supports both SMTP and sendmail transport. You'll learn about Rust's builder pattern (common in Rust APIs), working with TLS/SSL connections, and handling credentials securely.

This also introduces configuration management - SMTP settings should come from environment variables, not hardcoded values. The `config` or `dotenvy` crate helps manage this.

## What

**Goals:**
- Add `lettre` dependency with appropriate features (smtp, tokio runtime)
- Add `dotenvy` for environment variable management
- Create a `.env.example` file with required variables:
  - `SMTP_HOST`, `SMTP_PORT`, `SMTP_USERNAME`, `SMTP_PASSWORD`
  - `EMAIL_FROM`, `EMAIL_TO`
- Create `src/config/email.rs` to load email configuration
- Implement SMTP connection setup with TLS
- Handle authentication (PLAIN, LOGIN mechanisms)
- Test connection with a test email
- Never commit actual credentials (add `.env` to `.gitignore`)

**Acceptance Criteria:**
- [ ] `lettre` configured with SMTP transport
- [ ] Configuration loaded from environment variables
- [ ] `.env.example` documents required variables
- [ ] TLS encryption enabled for SMTP connection
- [ ] Can successfully connect to SMTP server
- [ ] Credentials are never hardcoded

## Resources

- [Lettre Documentation](https://docs.rs/lettre/latest/lettre/)
- [Lettre SMTP Guide](https://lettre.rs/sending-messages/)
- [dotenvy crate](https://docs.rs/dotenvy/latest/dotenvy/)
- [SMTP Authentication](https://www.samlogic.net/articles/smtp-commands-reference-auth.htm)
- [Gmail SMTP Settings](https://support.google.com/mail/answer/7126229) - if using Gmail
