// This module contains "adapters" - concrete implementation of ports
pub mod sqlite_repository;

// Re-export for convenience
pub use sqlite_repository::SqliteMantraRepository;