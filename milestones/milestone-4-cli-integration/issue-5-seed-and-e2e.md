# Add Seed Data and End-to-End Testing

**Milestone**: 4 - CLI & Integration

## Why

Before deploying, you need actual mantras to send and confidence that everything works together. Seed data lets you test the application realistically. End-to-end tests verify the entire application flow from CLI input to database to email.

This also teaches you about test databases, fixtures, and integration testing strategies.

## What

**Goals:**
- Create `src/db/seed.rs` with seed mantras:
  - At least 30-50 inspiring mantras
  - Various categories (motivation, mindfulness, gratitude, etc.)
- Add a `seed` CLI command to populate the database
- Write end-to-end test:
  - Seed database
  - Run send command
  - Verify email sent (using mock)
  - Verify history recorded
- Test edge cases:
  - What happens when all mantras are exhausted?
  - Database connection failures
  - SMTP failures
- Document the full workflow in README

**Acceptance Criteria:**
- [ ] Seed data with 30+ mantras
- [ ] `cargo run -- seed` populates database
- [ ] End-to-end test covers full workflow
- [ ] Edge cases tested
- [ ] Application ready for daily use
- [ ] README updated with usage instructions

## Resources

- [Database Seeding Patterns](https://edgeguides.rubyonrails.org/active_record_migrations.html#migrations-and-seed-data)
- [Integration Testing Strategies](https://martinfowler.com/bliki/IntegrationTest.html)
- [Writing Good Documentation](https://www.writethedocs.org/guide/writing/beginners-guide-to-docs/)
