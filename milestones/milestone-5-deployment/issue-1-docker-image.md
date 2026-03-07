# Create Dockerfile and Container Image

**Milestone**: 5 - Deployment

## Why

Containers solve the "works on my machine" problem. Docker packages your application with all its dependencies, making deployment to your Raspberry Pi predictable and repeatable. Rust's small binary size and musl static linking make it perfect for minimal container images.

You'll learn about multi-stage Docker builds to keep images small, cross-compilation for ARM architecture, and best practices for containerizing Rust applications.

## What

**Goals:**
- Create a `Dockerfile` with multi-stage build:
  - Stage 1: Build the Rust binary
  - Stage 2: Create minimal runtime image
- Use `rust:alpine` or `debian:slim` as base
- Cross-compile for ARM64 architecture (Raspberry Pi 4)
- Copy only the binary and runtime dependencies
- Set appropriate environment variables
- Create `.dockerignore` file
- Build and test the image locally
- Push to container registry (Docker Hub or GitHub Container Registry)
- Keep final image size under 50MB if possible

**Acceptance Criteria:**
- [ ] Multi-stage Dockerfile builds successfully
- [ ] Image cross-compiled for ARM64/aarch64
- [ ] Final image is minimal (only runtime dependencies)
- [ ] `.dockerignore` excludes build artifacts
- [ ] Image runs successfully (test with `docker run`)
- [ ] Image pushed to registry

## Resources

- [Rust Docker Official Image](https://hub.docker.com/_/rust)
- [Multi-stage Builds](https://docs.docker.com/build/building/multi-stage/)
- [Cross-compiling Rust for Raspberry Pi](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/)
- [Dockerfile Best Practices](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)
- [Building for ARM on x86](https://www.docker.com/blog/multi-arch-build-and-images-the-simple-way/)
