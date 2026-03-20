# mantra-generator
An application to send a daily mantra to a user, desgined to run on raspberry pi 4.


## Project goals

- A Rust CLI application triggered once per day
- Selects a “mantra of the day”
- Persists sent history so the same mantra is not repeated
- Sends the mantra via email
- Designed with clean separation between:
- pure domain logic
- persistence
- delivery mechanisms
- Architecture must support later adding an HTTP API without rewriting core logic

## Technical constraints

- Use idiomatic Rust
- Async where appropriate
- SQLite for persistence
- Email as the first delivery mechanism
- Configuration via environment variables
- Strong, explicit error handling
- Initial scope (MVP)
- Create a Cargo project structure
- Define domain models (Mantra, MantraId)
- Implement a pure function to select a mantra given a list and history
- Stub persistence and email layers (traits + placeholder implementations)
- Provide a CLI command (run) that performs one “daily” execution
- Include logging

## Preferred crates

- clap
- tokio
- sqlx (SQLite)
- lettre
- tracing
- thiserror

## Running Tests

 Run all the tests
 ```sh
 cargo test
 ```

 Run only the integration tests
 ```sh
 cargo test --test repository_tests
 ```