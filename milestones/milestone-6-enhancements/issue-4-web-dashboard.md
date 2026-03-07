# Create Web Dashboard

**Milestone**: 6 - Future Enhancements

## Why

A web dashboard provides a user-friendly interface for managing mantras and viewing history. While this issue focuses on frontend development (outside Rust), it demonstrates full-stack capabilities and creates a complete product.

You'll learn about serving static files in Rust, integrating frontend builds, and creating a complete web application.

## What

**Goals:**
- Choose frontend approach:
  - Simple: HTML templates with server-side rendering (Tera, Askama)
  - Modern: Separate SPA (React, Vue, Svelte) served by Axum
  - Rust frontend: Yew or Leptos (Rust WebAssembly framework)
- Create dashboard views:
  - List all mantras with search/filter
  - Add new mantra form
  - View sent history with calendar
  - Manual "send now" button
  - Statistics (total mantras, usage patterns)
- Implement CRUD operations via API
- Add responsive design (works on mobile)
- Serve static assets from Axum
- Deploy dashboard alongside API

**Acceptance Criteria:**
- [ ] Web dashboard accessible via browser
- [ ] Can view and manage mantras through UI
- [ ] History displayed attractively
- [ ] Manual send functionality works
- [ ] Responsive design (mobile-friendly)
- [ ] Integrated with API endpoints
- [ ] Deployed to Raspberry Pi alongside API

## Resources

- [Axum Static File Serving](https://docs.rs/tower-http/latest/tower_http/services/struct.ServeDir.html)
- [Tera Templates](https://docs.rs/tera/latest/tera/)
- [Askama Templates](https://docs.rs/askama/latest/askama/)
- [Yew Framework](https://yew.rs/) - Rust frontend framework
- [Leptos](https://www.leptos.dev/) - Modern Rust web framework
- [htmx](https://htmx.org/) - Simple interactive HTML
