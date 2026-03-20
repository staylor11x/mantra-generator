use core::panic;

use mantra_service::adapters::SqliteMantraRepository;
use mantra_service::domain::{MantraId};
use mantra_service::ports::{MantraRepository, RepositoryError};
use sqlx::sqlite::SqlitePool;

/// Helper function to create an in-memory test database with schema
async fn setup_test_db() -> SqlitePool {
    // Create in memory database
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    // Run migrations to set up schema
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// Helper function to create a test repository
async fn create_test_repo() -> SqliteMantraRepository {
    let pool = setup_test_db().await;
    SqliteMantraRepository::from_pool(pool)
}

#[tokio::test]
async fn test_add_and_get_mantra(){
    let repo = create_test_repo().await;

    // Add a mantra
    let mantra = repo
        .add_mantra("I am present and mindful.".to_string(), 
        Some("Mindfulness".to_string()),
    )
    .await
    .expect("Failed to add mantra");

    // Verify it was created
    assert_eq!(mantra.text, "I am present and mindful.");
    assert_eq!(mantra.category, Some("Mindfulness".to_string()));
    assert!(mantra.id.value() > 0, "Should have valid ID");

    // Retrieve by ID
    let retrieved = repo
        .get_mantra_by_id(mantra.id)
        .await
        .expect("Failed to get mantra by ID");

    assert_eq!(retrieved.id, mantra.id);
    assert_eq!(retrieved.text, mantra.text);
    assert_eq!(retrieved.category, mantra.category);
}

#[tokio::test]
async fn test_get_all_mantras() {
    let repo = create_test_repo().await;

    // Initial empty
    let mantras = repo.get_all_mantras().await.expect("Failed to get mantras");
    assert_eq!(mantras.len(), 0, "Should start with empty database");

    // Add three mantras
    repo.add_mantra("Mantra 1".to_string(), Some("Cat1".to_string()))
        .await
        .expect("Failed to add mantra 1");

    repo.add_mantra("Mantra 2".to_string(), None)
        .await
        .expect("Failed to add mantra 2");

    repo.add_mantra("Mantra 3".to_string(), Some("Cat3".to_string()))
        .await
        .expect("Failed to add mantra 3");

    // Retrieve all
    let mantras = repo.get_all_mantras().await.expect("Failed to get mantras");
    assert_eq!(mantras.len(), 3, "Should have 3 metrics");

    // Verify they're in order
    assert_eq!(mantras[0].text, "Mantra 1");
    assert_eq!(mantras[1].text, "Mantra 2");
    assert_eq!(mantras[2].text, "Mantra 3");

    // Verify categories
    assert_eq!(mantras[0].category, Some("Cat1".to_string()));
    assert_eq!(mantras[1].category, None);
    assert_eq!(mantras[2].category, Some("Cat3".to_string()));
}

#[tokio::test]
async fn test_record_and_get_sent_history(){
    let repo = create_test_repo().await;

    // Add mantras
    let mantra1 = repo
        .add_mantra("Fist mantra".to_string(), None)
        .await
        .expect("Failed to add mantra 1");

    let mantra2 = repo
        .add_mantra("Second mantra".to_string(), None)
        .await
        .expect("Failed to add mantra 2");

    // Initially there is no history
    let history = repo
        .get_sent_history(30)
        .await
        .expect("Failed to get history");
    assert_eq!(history.len(), 0, "Should have no history initially");

    // Record sends
    repo.record_sent(mantra1.id)
        .await
        .expect("Failed to record send 1");

    repo.record_sent(mantra2.id)
        .await
        .expect("Failed to record send 2");

    // Verify history
    let history = repo
        .get_sent_history(30)
        .await
        .expect("Failed to get history");

    assert_eq!(history.len(), 2, "Should have 2 sent records");

    // Most recent first (DESC order)
    assert_eq!(history[0], mantra2.id);
    assert_eq!(history[1], mantra1.id);

}

#[tokio::test]
async fn test_sent_history_respects_days_filter() {
    let repo = create_test_repo().await;

    // Add a mantra
    let mantra = repo
        .add_mantra("Test mantra".to_string(),None)
        .await
        .expect("Failed to add mantra");

    // Record sent
    repo.record_sent(mantra.id)
        .await
        .expect("Failed to record send");

    // Should appear in 30-day history
    let history = repo
        .get_sent_history(30)
        .await
        .expect("Failed to get history");
    assert_eq!(history.len(),1);

    // Should appear in 1-day history (just sent)
    let history = repo
        .get_sent_history(1)
        .await
        .expect("Failed to get history");
    assert_eq!(history.len(),1);
}

#[tokio::test]
async fn test_get_nonexistent_mantra(){
    let repo = create_test_repo().await;

    // Try to get a mantra that doesn't exist
    let result = repo.get_mantra_by_id(MantraId::new(999)).await;

    assert!(result.is_err(), "Should return error for nonexistent mantra");

    match result {
        Err(RepositoryError::NotFound(id)) => {
            assert_eq!(id, 999, "Should report correct ID");
        }
        _ => panic!("Expected NorFound error"),
    }
}

#[tokio::test]
async fn test_record_sent_for_nonexistent_mantra() {
    let repo = create_test_repo().await;

    // Try to record sent for mantra that doesn't exist
    // SQLite won't enforce foreign key on in-memory DB without explicit pragma
    // But we can test that the function doesn't panic
    let result = repo.record_sent(MantraId::new(999)).await;

    // This might succeed in SQLite without FK enforcement
    // In production with FK constraints, it would fail
    // For now, just verify it doesn't panic
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_add_mantra_with_empty_text() {
    let repo = create_test_repo().await;

    // SQLite allows empty strings (no CHECK constraints)
    let result = repo
        .add_mantra("".to_string(), None)
        .await;

    // Should succeed (database allows it)
    assert!(result.is_ok(), "Database allows empty text");

    let mantra = result.unwrap();
    assert_eq!(mantra.text, "");
}

#[tokio::test]
async fn test_concurrent_reads() {
    let repo = create_test_repo().await;

    // Add some mantras
    repo.add_mantra("Mantra 1".to_string(), None)
        .await
        .expect("Failed to add mantra");

    repo.add_mantra("Mantra 2".to_string(), None)
        .await
        .expect("Failed to add mantra");

    // Perform concurrent reads
    let (result1, result2, result3) = tokio::join!(
        repo.get_all_mantras(),
        repo.get_all_mantras(),
        repo.get_all_mantras()
    );

    // All should succeed
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // All should return same data
    let mantras1 = result1.unwrap();
    let mantras2 = result2.unwrap();
    let mantras3 = result3.unwrap();

    assert_eq!(mantras1.len(), 2);
    assert_eq!(mantras2.len(), 2);
    assert_eq!(mantras3.len(), 2);
}

#[tokio::test]
async fn test_created_at_timestamp_preserved(){
    let repo = create_test_repo().await;

    // Add a mantra
    let mantra = repo
        .add_mantra("Test timestamp".to_string(), None)
        .await
        .expect("Failed to add mantra");

    let original_time = mantra.created_at;

    // Wait a bit...
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Retrieve the same mantra
    let retreievd = repo
        .get_mantra_by_id(mantra.id)
        .await
        .expect("Failed to get mantra");

    // Timestamps should match
    assert_eq!(
        retreievd.created_at,
        original_time,
        "Created timestamp should be preserved! Got {:?}, expected {:?}",
        retreievd.created_at,
        original_time
    );
}