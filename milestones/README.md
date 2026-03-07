# Mantra Generator - Project Milestones

This directory contains all the milestones and issues for building the Mantra Generator application. Each milestone represents a major phase of development, and each issue within a milestone is designed to be completable in approximately 1 hour.

## Milestone Overview

### Milestone 1: Core Domain & Project Foundation
**Focus**: Learn Rust fundamentals and domain-driven design
- Issue 1: Setup Cargo project with dependencies
- Issue 2: Define core domain models
- Issue 3: Implement mantra selection logic
- Issue 4: Write unit tests for domain logic

**Key Learning**: Cargo, structs, enums, traits, ownership, Result/Option types, testing

---

### Milestone 2: Persistence Layer
**Focus**: Database integration and hexagonal architecture
- Issue 1: Design database schema and migrations
- Issue 2: Implement repository trait (port)
- Issue 3: Implement SQLite repository adapter
- Issue 4: Add integration tests for persistence

**Key Learning**: SQLx, async/await, traits, compile-time SQL verification, ports & adapters pattern

---

### Milestone 3: Email Delivery
**Focus**: External service integration and configuration
- Issue 1: Implement email service trait (port)
- Issue 2: Configure SMTP with lettre
- Issue 3: Implement email service adapter
- Issue 4: Add email service tests

**Key Learning**: Lettre, SMTP, environment configuration, mocking, trait objects

---

### Milestone 4: CLI & Integration
**Focus**: Bringing all components together
- Issue 1: Build CLI with clap
- Issue 2: Add structured logging with tracing
- Issue 3: Wire components together (dependency injection)
- Issue 4: Implement command handlers and main loop
- Issue 5: Add seed data and end-to-end testing

**Key Learning**: Clap, tracing, Arc for shared state, dependency injection, integration testing

---

### Milestone 5: Deployment
**Focus**: Production deployment and operations
- Issue 1: Create Dockerfile and container image
- Issue 2: Set up systemd timer or cron job
- Issue 3: Deploy to Raspberry Pi
- Issue 4: Production hardening and monitoring

**Key Learning**: Docker, cross-compilation, Linux services, systemd, production best practices

---

### Milestone 6: Future Enhancements
**Focus**: Extending the application with new features
- Issue 1: Design HTTP API architecture
- Issue 2: Implement REST API with Axum
- Issue 3: Add authentication and authorization
- Issue 4: Create web dashboard
- Issue 5: Additional enhancements and learning opportunities

**Key Learning**: Web frameworks, REST API design, authentication, frontend integration

---

## Using the Issues

### Creating GitHub Issues

Each issue markdown file can be uploaded to GitHub using the `create-issue.sh` script:

```bash
# Create a single issue
./create-issue.sh milestones/milestone-1-foundation/issue-1-setup-project.md

# With custom labels
./create-issue.sh milestones/milestone-1-foundation/issue-1-setup-project.md "learning,rust"

# With assignee
./create-issue.sh milestones/milestone-1-foundation/issue-1-setup-project.md "learning" "@me"
```

### Batch Upload

To upload all issues at once:

```bash
# From the project root
for file in milestones/**/*.md; do
  ./create-issue.sh "$file" "learning"
done
```

### Issue Format

Each issue follows this structure:
- **Title**: First H1 heading
- **Milestone**: Metadata indicating which milestone the issue belongs to
- **Why**: Educational context explaining the learning goals
- **What**: Concrete goals and acceptance criteria
- **Resources**: Links to documentation and learning materials

### Recommended Workflow

1. **Start with Milestone 1**: Complete all issues in order
2. **One issue at a time**: Each issue builds on previous work
3. **Test thoroughly**: Run tests after each issue completion
4. **Commit frequently**: Commit after completing each issue
5. **Read the "Why"**: Understanding context is as important as the implementation
6. **Use the resources**: Don't hesitate to dive deep into documentation
7. **Experiment**: Try variations and explore beyond the requirements

### Milestone Dependencies

- **Milestone 1** → **Milestone 2** → **Milestone 3** → **Milestone 4**: Must be completed in order
- **Milestone 5**: Requires Milestone 4 to be complete
- **Milestone 6**: Optional enhancements, can be done anytime after Milestone 4

### Estimated Timeline

- **Milestone 1**: 4 hours (4 issues × 1 hour)
- **Milestone 2**: 4 hours (4 issues × 1 hour)
- **Milestone 3**: 4 hours (4 issues × 1 hour)
- **Milestone 4**: 5 hours (5 issues × 1 hour)
- **Milestone 5**: 4 hours (4 issues × 1 hour)
- **Milestone 6**: 5+ hours (optional, at your own pace)

**Total core development**: ~21 hours
**With enhancements**: 26+ hours

### Tips for Success

1. **Don't skip the "Why" sections**: They provide crucial learning context
2. **Follow acceptance criteria**: They ensure you've completed the issue properly
3. **Consult resources**: The links are carefully selected for your learning
4. **Ask for help**: Rust has a fantastic community (Rust forums, Discord, Reddit)
5. **Take breaks**: Learning Rust can be challenging; take time to let concepts sink in
6. **Celebrate milestones**: Each completed milestone is a significant achievement!

### Getting Help

- **Rust Forums**: https://users.rust-lang.org/
- **Rust Discord**: https://discord.gg/rust-lang
- **Rust Subreddit**: https://www.reddit.com/r/rust/
- **This Week in Rust**: https://this-week-in-rust.org/

---

## Learning Path

This project teaches you:

✅ **Core Rust**: Ownership, borrowing, lifetimes, traits, enums, pattern matching  
✅ **Async Rust**: Tokio, async/await, futures  
✅ **Database**: SQLx, migrations, compile-time query verification  
✅ **Web Development**: HTTP APIs, JSON, authentication  
✅ **Architecture**: Hexagonal architecture, dependency injection, ports & adapters  
✅ **Testing**: Unit tests, integration tests, mocking  
✅ **DevOps**: Docker, containerization, Linux services, deployment  
✅ **Production**: Logging, monitoring, error handling, security  

Enjoy your Rust learning journey! 🦀
