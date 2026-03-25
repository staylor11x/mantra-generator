use crate::domain::Mantra;
use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during email operations
#[derive(Error,Debug)]
pub enum EmailError {
    #[error("Failed to connect to SMTP server: {0}")]
    ConnectionError(String),

    #[error("SMTP authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Failed to send email: {0}")]
    SendError(String),

    #[error("Invalid email address: {0}")]
    InvalidRecipient(String),

    #[error("Failed to build email message: {0}")]
    MessageBuildError(String),
}

// Helper to convert from lettre errors
impl From<lettre::error::Error> for EmailError {
    fn from(err: lettre::error::Error) -> Self {
        // Lettre's error type doesn't expose variants directly
        // Convert to string representation
        EmailError::SendError(err.to_string())
    }
}

impl From<lettre::address::AddressError> for EmailError {
    fn from(err: lettre::address::AddressError) -> Self {
        EmailError::InvalidRecipient(err.to_string())
    }
}

impl From<lettre::transport::smtp::Error> for EmailError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        if err.is_client() {
            EmailError::ConnectionError(err.to_string())
        } else if err.is_transient() {
            EmailError::SendError(format!("Temporary failure: {}", err))
        } else if err.is_permanent() {
            EmailError::SendError(format!("Permanent failure: {}", err))
        } else {
            EmailError::SendError(err.to_string())
        }
    }
}

/// Email service trait for sending mantras
/// 
/// This is a "port" in hexagonal architecture - it defines what email operations
/// are needed without specifying the implementation. The actual implementation 
/// (adapter) can use SMTP, SendGrid, AWS SES, or a mock for testing.
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send a mantra to a recipient via email
    /// 
    /// # Arguments
    /// * `recipient` - Email address to send to (e.g., "user@example.com")
    /// * `mantra` - The mantra to send
    /// 
    /// # Returns
    /// * `OK()` - Email sent successfully
    /// * `Err(EmailError)` - If sending fails
    /// 
    /// # Examples
    /// ```ignore
    /// let mantra = Mantra::new_now(/*... */);
    /// email_service.send_mantra("user@example.com", &mantra).await?;
    /// ```
    async fn send_mantra(&self, recipient: &str, mantra: &Mantra) -> Result<(), EmailError>;

    /// Test the email service connection and authentication
    /// 
    /// This is useful for validating configuration before attempting to send.
    /// 
    /// # Returns 
    /// * `Ok(())` - Connection successful and authenticated
    /// * `Err(EmailError)` - If connection or authentication fails
    async fn test_connection(&self) -> Result<(), EmailError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time check that trait is object safe
    #[allow(dead_code)]
    fn assert_object_safe(_service: &dyn EmailService) {}
}