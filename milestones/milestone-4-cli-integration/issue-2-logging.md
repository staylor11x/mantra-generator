# Add Structured Logging with Tracing

**Milestone**: 4 - CLI & Integration

## Why

`tracing` is more powerful than simple logging - it provides structured, contextual logging that's essential for debugging async applications. Unlike Go's standard `log` package, `tracing` understands async contexts and can track events across async boundaries.

You'll learn about spans (representing periods of time), events (point-in-time occurrences), and subscribers (how logs are output). This is crucial for production debugging on your Raspberry Pi.

## What

**Goals:**
- Add `tracing` and `tracing-subscriber` dependencies
- Initialize tracing in `main()` before running commands
- Add tracing spans to major operations:
  - Database queries
  - Email sending
  - Mantra selection
- Use different log levels: `trace!`, `debug!`, `info!`, `warn!`, `error!`
- Add structured fields to logs: `info!(mantra_id = %id, "Sending mantra")`
- Configure output format (JSON for production, pretty for development)
- Add `RUST_LOG` environment variable support for log level control
- Include timing information for operations

**Acceptance Criteria:**
- [ ] Tracing initialized in main function
- [ ] Spans added to major operations
- [ ] Structured logging with contextual fields
- [ ] Different log levels used appropriately
- [ ] `RUST_LOG` environment variable controls verbosity
- [ ] Logs include timing information for async operations

## Resources

- [Tracing Documentation](https://docs.rs/tracing/latest/tracing/)
- [Tracing Subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)
- [Tokio Tracing Guide](https://tokio.rs/tokio/topics/tracing)
- [Structured Logging](https://www.honeycomb.io/blog/structured-logging-and-your-team)
- [Log Levels Best Practices](https://stackoverflow.com/questions/2031163/when-to-use-the-different-log-levels)
