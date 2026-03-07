# Design HTTP API Architecture

**Milestone**: 6 - Future Enhancements

## Why

This demonstrates the power of your hexagonal architecture - you can add a new delivery mechanism (HTTP API) without touching your domain logic. You'll learn about API design, REST principles, and how to expose your existing business logic through a web interface.

This is valuable experience for microservices architecture and building APIs in Rust.

## What

**Goals:**
- Design RESTful API endpoints:
  - `GET /api/mantras` - List all mantras
  - `POST /api/mantras` - Add new mantra
  - `GET /api/mantras/{id}` - Get specific mantra
  - `GET /api/history` - Get sent history
  - `POST /api/send` - Manually trigger sending
  - `GET /health` - Health check
- Define request/response models with `serde`
- Plan authentication strategy (API keys, JWT, etc.)
- Design error response format
- Document API with OpenAPI/Swagger
- Choose web framework (Axum recommended for Tokio integration)
- Plan versioning strategy (`/api/v1/...`)

**Acceptance Criteria:**
- [ ] API endpoints defined in documentation
- [ ] Request/response models designed with serde
- [ ] Authentication strategy decided
- [ ] OpenAPI specification created
- [ ] Framework selected and added to dependencies
- [ ] Architecture documented (no implementation yet)

## Resources

- [Axum Web Framework](https://docs.rs/axum/latest/axum/)
- [RESTful API Design](https://restfulapi.net/)
- [OpenAPI Specification](https://swagger.io/specification/)
- [API Security Best Practices](https://owasp.org/www-project-api-security/)
- [Serde for JSON](https://serde.rs/)
