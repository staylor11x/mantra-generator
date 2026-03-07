# Production Hardening and Monitoring

**Milestone**: 5 - Deployment

## Why

A production system needs observability, error recovery, and maintenance procedures. You'll learn about production best practices: health checks, error alerting, backup strategies, and handling failures gracefully.

This transforms your project from a working prototype to a reliable production service.

## What

**Goals:**
- Implement health check endpoint or command
- Set up database backup strategy:
  - Regular SQLite backups
  - Backup rotation
- Configure failure handling:
  - What happens if email fails? (retry logic, alerting)
  - What if database is locked?
- Add monitoring/alerting:
  - Simple: email notification on failures
  - Advanced: Prometheus metrics, health dashboard
- Document recovery procedures:
  - How to restore from backup
  - How to restart services
  - How to update the application
- Set up update strategy:
  - Blue-green deployment or simple restart
  - Database migration handling
- Resource monitoring (CPU, memory, disk space)

**Acceptance Criteria:**
- [ ] Database backup script running regularly
- [ ] Failure recovery procedures documented
- [ ] Error notifications configured (email or monitoring)
- [ ] Update procedure documented and tested
- [ ] Resource usage monitored
- [ ] Runbook created for common operations

## Resources

- [SQLite Backup API](https://www.sqlite.org/backup.html)
- [Systemd OnFailure](https://www.freedesktop.org/software/systemd/man/systemd.unit.html#OnFailure=)
- [Prometheus Rust Client](https://docs.rs/prometheus/latest/prometheus/)
- [Site Reliability Engineering](https://sre.google/sre-book/table-of-contents/)
- [Creating Runbooks](https://www.atlassian.com/incident-management/devops/runbooks)
