# Design Database Schema and Migrations

**Milestone**: 2 - Persistence Layer

## Why

SQLite is perfect for small applications like this - it's embedded, serverless, and requires no setup. Unlike many Go ORMs, `sqlx` provides compile-time verification of your SQL queries against your database schema, catching errors before runtime.

Migrations ensure your database schema evolves in a controlled, repeatable way. This is especially important when deploying to your Raspberry Pi - migrations will automatically set up the database on first run.

## What

**Goals:**
- Install `sqlx-cli` tool: `cargo install sqlx-cli --features sqlite`
- Initialize sqlx: `sqlx database create` and `sqlx migrate add init`
- Design schema for two tables:
  - `mantras` table (id, text, category, created_at)
  - `sent_history` table (id, mantra_id, sent_at)
- Write SQL migrations in the generated migration files
- Learn about SQLite data types (INTEGER, TEXT, REAL, BLOB)
- Understand foreign key constraints in SQLite
- Run migrations: `sqlx migrate run`
- Set `DATABASE_URL` environment variable

**Acceptance Criteria:**
- [ ] `migrations/` directory contains `.sql` files
- [ ] Schema includes appropriate constraints (PRIMARY KEY, NOT NULL, FOREIGN KEY)
- [ ] Migrations run successfully with `sqlx migrate run`
- [ ] Database file is created (default `mantra.db`)
- [ ] Indexes added for common queries (e.g., on `sent_at`)

## Resources

- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [SQLx CLI Guide](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [SQLite Data Types](https://www.sqlite.org/datatype3.html)
- [SQLite Foreign Keys](https://www.sqlite.org/foreignkeys.html)
- [Database Migrations Best Practices](https://www.postgresql.org/docs/current/ddl-alter.html)
