# Add Authentication and Authorization

**Milestone**: 6 - Future Enhancements

## Why

Public APIs need authentication to prevent abuse. You'll learn about different auth strategies, how to implement middleware in Rust web frameworks, and secure API design. This is essential knowledge for any production API.

You'll also explore Rust's type system for encoding security constraints - making it impossible to accidentally skip authentication checks.

## What

**Goals:**
- Choose authentication method:
  - API Key (simplest, good for single user)
  - JWT tokens (more scalable)
- Implement authentication middleware:
  - Extract credentials from headers
  - Validate credentials
  - Attach user context to request
- Store credentials securely:
  - Hash API keys
  - Use environment variables
- Implement authorization:
  - Some endpoints public (health check)
  - Some require authentication (send mantra)
- Add rate limiting to prevent abuse
- Use type-safe extractor pattern (authenticated vs unauthenticated)
- Document authentication in API docs

**Acceptance Criteria:**
- [ ] Authentication middleware implemented
- [ ] Protected endpoints require valid credentials
- [ ] Credentials stored securely (hashed, in env vars)
- [ ] Rate limiting configured
- [ ] Type system enforces auth requirements
- [ ] Authentication documented in OpenAPI spec
- [ ] Tests for auth success and failure cases

## Resources

- [JWT in Rust](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)
- [Axum Middleware](https://docs.rs/axum/latest/axum/middleware/index.html)
- [API Key Authentication](https://www.httpwatch.com/httpgallery/authentication/)
- [Tower Rate Limiting](https://docs.rs/tower/latest/tower/limit/index.html)
- [OWASP API Security](https://owasp.org/www-project-api-security/)
