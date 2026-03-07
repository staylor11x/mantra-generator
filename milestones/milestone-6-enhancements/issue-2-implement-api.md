# Implement REST API with Axum

**Milestone**: 6 - Future Enhancements

## Why

`Axum` is a modern web framework built on Tokio, designed by the Tokio team. It's type-safe, has excellent performance, and integrates seamlessly with your existing async code. You'll learn about extractors, middleware, routing, and state management in Rust web applications.

This teaches you practical web development in Rust, a growing use case for the language.

## What

**Goals:**
- Add `axum`, `tower`, `tower-http` dependencies
- Create `src/api/` module structure
- Implement route handlers reusing domain logic:
  - Handlers call existing repository and service traits
  - Minimal logic in handlers (thin controller pattern)
- Set up application state with `Arc<AppContext>`
- Implement JSON serialization/deserialization
- Add request validation
- Implement proper HTTP status codes (200, 201, 400, 404, 500)
- Add CORS middleware if needed
- Create server startup code
- Make API run alongside or instead of CLI

**Acceptance Criteria:**
- [ ] All planned endpoints implemented
- [ ] Handlers use existing domain logic (no duplication)
- [ ] Proper HTTP status codes for success/error cases
- [ ] JSON request/response handling
- [ ] State management with Arc
- [ ] Server starts and responds to requests
- [ ] Integration with existing repository and services

## Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Tower Middleware](https://docs.rs/tower/latest/tower/)
- [HTTP Status Codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
- [REST API Error Handling](https://blog.restcase.com/rest-api-error-handling-best-practices/)
