# Set Up Systemd Timer or Cron Job

**Milestone**: 5 - Deployment

## Why

Linux systems use either cron or systemd timers to schedule recurring tasks. Systemd timers are more modern and provide better logging and dependency management. You'll learn about Linux service management, which is fundamental DevOps knowledge.

This is what makes your application run automatically every day without manual intervention.

## What

**Goals:**
- Choose between systemd timer (recommended) or cron
- If using systemd:
  - Create `.service` file defining the application execution
  - Create `.timer` file defining the schedule
  - Set up to run daily (e.g., 8:00 AM)
- If using cron:
  - Create crontab entry for daily execution
- Configure logging output (journalctl for systemd, file for cron)
- Set appropriate user permissions
- Handle environment variables securely
- Test the timer/cron locally before deploying
- Document the schedule configuration

**Acceptance Criteria:**
- [ ] Timer/cron configuration files created
- [ ] Scheduled to run once daily at specified time
- [ ] Proper logging configuration
- [ ] Environment variables passed securely
- [ ] Tested locally on Linux (WSL, VM, or Pi)
- [ ] Documentation for modifying schedule

## Resources

- [Systemd Timers](https://www.freedesktop.org/software/systemd/man/systemd.timer.html)
- [Systemd Service Files](https://www.freedesktop.org/software/systemd/man/systemd.service.html)
- [Cron Expression Guide](https://crontab.guru/)
- [Managing Systemd Services](https://www.digitalocean.com/community/tutorials/how-to-use-systemctl-to-manage-systemd-services-and-units)
- [Cron vs Systemd Timers](https://opensource.com/article/20/7/systemd-timers)
