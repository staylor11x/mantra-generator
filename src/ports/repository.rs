use thiserror::Error;

/// Errors that can occur during repository operations
#[derive(Error,Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Mantra not found with id: {0}")]
    NotFound(i64),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),
}

// Helper to convert from sqlx errors
impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(0),
            sqlx::Error::Database(db_err) => {
                RepositoryError::DatabaseError(db_err.to_string())
            }
            _ => RepositoryError::DatabaseError(err.to_string()),
        }
    }
}

use crate::domain::{Mantra,MantraId};
use async_trait::async_trait;


/// Repository trait for mantra persistence operations
/// 
/// This is a "port" in hexagonal architecture - it defines what operations
/// are needed without specifying how they're implemented. The actual
/// implementation (adapter) can use SQLite, PostgreSQL, or even in-memory storage!
#[async_trait]
pub trait MantraRepository: Send + Sync {
    /// Get all available mantras from the repository
    /// 
    /// # Returns 
    /// * `Ok(Vec<Mantra>` - All mantras in the database
    /// * `Err(RepositoryError)` - If database query fails
    async fn get_all_mantras(&self) -> Result<Vec<Mantra>, RepositoryError>;

    /// Get mantra IDs that were sent within the last N days
    /// 
    /// # Arguments
    /// * `days` - Number of days to look back in history
    /// 
    /// # Returns
    /// * `OK(Vec<MantraId>` - List of mantra IDs send within the time window
    /// * `Err(RepositoryError)` - If database query fails
    async fn get_sent_history(&self, days: i32) -> Result<Vec<MantraId>, RepositoryError>;

    /// Record that a mantra was sent
    /// 
    /// # Arguments
    /// * `mantra_id` - The ID of the mantra that was sent
    /// 
    /// # Returns
    /// * `Ok(())` - Record successfully created
    /// * `Err(RepositoryError)` - If inset fails or mantra doesn't exist
    async fn record_sent(&self, mantra_id: MantraId) -> Result<(), RepositoryError>;

    /// Add a new mantra to the repository
    /// 
    /// # Arguments
    /// * `text` - The mantra text content
    /// * `category` - Optional category for the mantra
    /// 
    /// # Returns
    /// * `OK(Mantra)` - The newly created mantra with its assigned ID
    /// * `Err(RepositoryError)` - If insert fails
    async fn add_mantra(
        &self, 
        text: String,
        category: Option<String>,
    ) -> Result<Mantra,RepositoryError>;

    /// Get a specific mantra by ID
    /// 
    /// # Arguments
    /// * `id` - The mantra ID to look up
    /// 
    /// # Returns
    /// * `OK(Mantra)` - The mantra with the given ID
    /// * `Err(RepositoryError::NotFound)` - If mantra doesn't exist
    async fn get_mantra_by_id(&self, id: MantraId) -> Result<Mantra,RepositoryError>;
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn assert_object_safe(_repo: &dyn MantraRepository) {}
}

