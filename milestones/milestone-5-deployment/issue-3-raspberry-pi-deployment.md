# Deploy to Raspberry Pi

**Milestone**: 5 - Deployment

## Why

This is where everything comes together on real hardware. You'll learn about remote deployment, managing services on embedded Linux, and troubleshooting in a production environment. This experience is valuable for IoT and edge computing scenarios.

## What

**Goals:**
- Set up Raspberry Pi OS (64-bit for better Rust compatibility)
- Install Docker on Raspberry Pi (or copy binary directly)
- Transfer application to Pi:
  - Via Docker image pull
  - Or via direct binary copy
- Set up SQLite database directory
- Configure environment variables securely:
  - Use systemd environment files or `.env`
  - Never expose credentials in service files
- Install and enable systemd timer
- Verify application runs successfully
- Set up log rotation
- Document deployment steps
- Test receiving first daily mantra

**Acceptance Criteria:**
- [ ] Application deployed to Raspberry Pi
- [ ] Database initialized and persisted
- [ ] Environment variables configured securely
- [ ] Systemd timer running and enabled on boot
- [ ] Logs accessible via journalctl or log files
- [ ] Successfully received test mantra via email
- [ ] Deployment documented for future updates

## Resources

- [Raspberry Pi Documentation](https://www.raspberrypi.org/documentation/)
- [Deploying Docker to Raspberry Pi](https://www.docker.com/blog/happy-pi-day-docker-raspberry-pi/)
- [SSH Key Authentication](https://www.ssh.com/academy/ssh/public-key-authentication)
- [Systemd Environment Files](https://www.freedesktop.org/software/systemd/man/systemd.exec.html#EnvironmentFile=)
- [Linux Log Rotation](https://www.man7.org/linux/man-pages/man8/logrotate.8.html)
