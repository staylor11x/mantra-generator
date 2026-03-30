use std::time::Duration;
use thiserror::Error;

/// Errors that can occur wen loading email configuration
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVariable(String),

    #[error("Invalid port number: {0}")]
    InvalidPort(String),

    #[error("Invalid timeout value: {0}")]
    InvalidTimeout(String),

    #[error("Failed to load .env file: {0}")]
    DotenvError(#[from] dotenvy::Error),
}

/// Email configuration loaded from environment variables
/// 
/// This struct holds all SMTP settings needed to connect and send emails
/// All sensitive data (passwords) should only come from environment variables,
/// never hardcoded in source code!
#[derive(Debug, Clone)]
pub struct EmailConfig {

    /// SMTP server hostname (e.g., "smtp.gmail.com")
    pub smtp_host: String,

    /// SMTP server port (typically 587 for TLS, 465 for SSL)
    pub smtp_port: u16,

    /// SMTP username
    pub smtp_username: String,

    /// SMTP password (use app-specific passwords)
    /// 
    /// WARNING: This field contains sensitive data. Never log or print it!
    smtp_password: String,

    /// Email address to send from
    pub email_from: String,

    /// Email address to send to 
    pub email_to: String,

    /// Connection timeout
    pub timeout: Duration,
}

impl EmailConfig {
    /// Load configuration from environment variables
    /// 
    /// This will attempt to load a `.env` file if present, then read
    /// the required environment variables. If any required variable is 
    /// missing, returns an error.
    /// 
    /// # Required environment variables
    /// - `SMTP_HOST` - SMTP server hostname
    /// - `SMTP_PORT` - SMTP server port (must be valid u16)
    /// - `SMTP_USERNAME` - SMTP authentication username
    /// - `SMTP_PASSWORD` - SMTP authentication password
    /// - `EMAIL_FROM` - Sender email address
    /// - `EMAIL_TO` - Recipient email address
    /// 
    /// # Optional Environment Variables
    /// - `SMTP_TIMEOUT` - Connection timeout in seconds (default: 60)
    /// 
    /// # Errors
    /// Returns `ConfigError` if:
    /// - Any required variable is missing
    /// - Port number is invalid
    /// - Timeout value is invalid
    /// 
    /// # Example
    /// ```no_run
    /// use mantra_service::config::EmailConfig;
    /// 
    /// let config = EmailConfig::from_env().expect("Failed to load email config");
    /// println!("Connecting to {}", config.smtp_host);
    /// ```
    pub fn from_env() -> Result<Self,ConfigError> {
        // Try to load .env file (it's ok if it doesn't exist)
        // In production, you'd use actual environment variables
        let _ = dotenvy::dotenv();

        // Load required variables
        let smtp_host = std::env::var("SMTP_HOST")
            .map_err(|_| ConfigError::MissingVariable("SMTP_HOST".to_string()))?;

        let smtp_port_str = std::env::var("SMTP_PORT")
            .map_err(|_| ConfigError::MissingVariable("SMTP_PORT".to_string()))?;

        let smtp_port = smtp_port_str
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort(smtp_port_str))?;

        let smtp_username = std::env::var("SMTP_USERNAME")
            .map_err(|_| ConfigError::MissingVariable("SMTP_USERNAME".to_string()))?;

        let smtp_password = std::env::var("SMTP_PASSWORD")
            .map_err(|_| ConfigError::MissingVariable("SMTP_PASSWORD".to_string()))?;

        let email_from = std::env::var("EMAIL_FROM")
            .map_err(|_| ConfigError::MissingVariable("EMAIL_FROM".to_string()))?;

        let email_to = std::env::var("EMAIL_TO")
            .map_err(|_| ConfigError::MissingVariable("EMAIL_TO".to_string()))?;

        let timeout = match std::env::var("SMTP_TIMEOUT") {
            Ok(timeout_str) => {
                let seconds = timeout_str
                    .parse::<u64>()
                    .map_err(|_| ConfigError::InvalidTimeout(timeout_str))?;
                Duration::from_secs(seconds)
            }
            Err(_) => Duration::from_secs(60),
        };

        Ok(EmailConfig {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            email_to,
            email_from,
            timeout,
        })
    }

    /// Get the SMTP password
    /// 
    /// This method provide controlled access to the password
    /// The password is not public to prevent accidental logging
    pub fn smtp_password(&self) -> &str {
        &self.smtp_password
    }
}

// Custom Debug implementation to avoid accidentally printing passwords
impl std::fmt::Display for EmailConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EmailConfig {{ smtp_host: {}, smtp_port: {}, smtp_username: {}, email_from: {}, email_to: {} }}",
            self.smtp_host, self.smtp_port, self.smtp_username, self.email_from, self.email_to
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display(){
        let err = ConfigError::MissingVariable("SMTP_HOST".to_string());
        assert_eq!(err.to_string(), "Missing required environment variable: SMTP_HOST");
    }

    #[test]
    fn test_email_config_display_no_password(){
        let config = EmailConfig {
            smtp_host: "smtp.example.com".to_string(),
            smtp_port: 587,
            smtp_username: "user@example.com".to_string(),
            smtp_password: "secret123".to_string(),
            email_from: "from@example.com".to_string(),
            email_to: "to@example.com".to_string(),
            timeout: Duration::from_secs(60),
        };

        let display = format!("{}", config);
        assert!(!display.contains("secret123"), "Password should not appear in the Display output");
        assert!(display.contains("smtp.example.com"));
    }
}

