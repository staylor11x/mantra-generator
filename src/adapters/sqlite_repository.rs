use crate::domain::{Mantra, MantraId};
use crate::ports::{MantraRepository, RepositoryError};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePool, FromRow};

/// SQLite implementation of the MantraRepository trait
pub struct SqliteMantraRepository {
    pool: SqlitePool,
}

impl SqliteMantraRepository {
    /// Create a new repository with the given database URL
    ///
    /// # Arguments
    /// * `database_url` - SQLite connection string (e.g., "sqlite:mantra.db")
    ///
    /// # Returns
    /// * `Ok(Self)` - Successfully connected repository
    /// * `Err(RepositoryError)` - If connection fails
    pub async fn new(database_url: &str) -> Result<Self, RepositoryError> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| RepositoryError::ConnectionError(e.to_string()))?;

        Ok(Self { pool })
    }

    /// Create a repository from an existing pool
    /// Useful for testing and when pool is managed elsewhere
    pub fn from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct MantraRow {
    id: i64,
    text: String,
    category: Option<String>,
    created_at: String,
}

impl From<MantraRow> for Mantra {
    fn from(row: MantraRow) -> Self {
        // Parse the SQLite TEXT timestamp to DateTime<Utc>
        let created_at = DateTime::parse_from_rfc3339(&row.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Mantra::new(
            MantraId::new(row.id),
            row.text,
            row.category,
            created_at,
        )
    }
}

#[async_trait]
impl MantraRepository for SqliteMantraRepository {
    async fn get_all_mantras(&self) -> Result<Vec<Mantra>, RepositoryError> {
        let rows = sqlx::query_as::<_,MantraRow>(
            "SELECT id, text, category, created_at FROM mantras ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Mantra::from).collect())
    }

    async fn get_sent_history(&self, days: i32) -> Result<Vec<MantraId>, RepositoryError> {
        // Calculate the cutoff date
        let cutoff = format!(
            "datetime('now', '-{} days')",
            days
        );

        let query = format!(
            "SELECT mantra_id FROM sent_history WHERE sent_at >= {} ORDER BY sent_at DESC",
            cutoff
        );

        let rows = sqlx::query_scalar::<_,i64>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(MantraId::new).collect())
    }

    async fn record_sent(&self, mantra_id: MantraId) -> Result<(), RepositoryError> {
        sqlx::query(
            "INSERT INTO sent_history (mantra_id, sent_at) VALUES (?1, datetime('now'))"
        )
        .bind(mantra_id.value())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn add_mantra(&self, text: String, category: Option<String>) -> Result<Mantra, RepositoryError> {
        let result = sqlx::query(
            "INSERT INTO mantras (text, category, created_at) VALUES (?1, ?2, datetime('now'))"
        )
        .bind(&text)
        .bind(&category)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid();

        // Fetch the newly created mantra to get the exact created_at
        self.get_mantra_by_id(MantraId::new(id)).await
    }

    async fn get_mantra_by_id(&self, id: MantraId) -> Result<Mantra, RepositoryError> {
        let row = sqlx::query_as::<_, MantraRow>(
            "SELECT id, text, category, created_at FROM mantras WHERE id = ?1"
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Mantra::from(row)),
            None => Err(RepositoryError::NotFound(id.value()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_repository_creation() {
        // This tests verifies we can create a repository
        // Full tests will be in the integration tests
        let result = SqliteMantraRepository::new("sqlite::memory:").await;
        assert!(result.is_ok(), "Should be able to create in-memory database");
    }
}