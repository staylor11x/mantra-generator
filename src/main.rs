use mantra_service::adapters::SqliteMantraRepository;
use mantra_service::ports::MantraRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")?;
    let repo = SqliteMantraRepository::new(&database_url).await?;
    
    // Test: Add a mantra
    let mantra = repo.add_mantra(
        "I am capable of amazing things.".to_string(),
        Some("Motivation".to_string()),
    ).await?;
    
    println!("Created: {} (ID: {})", mantra.text, mantra.id);
    
    // Test: Get all mantras
    let all_mantras = repo.get_all_mantras().await?;
    println!("\nAll mantras: {}", all_mantras.len());
    for m in &all_mantras {
        println!("  - {}", m);
    }
    
    // Test: Record sent
    repo.record_sent(mantra.id).await?;
    println!("\nRecorded send for {}", mantra.id);
    
    // Test: Get history
    let history = repo.get_sent_history(30).await?;
    println!("Sent history (last 30 days): {:?}", history);
    
    Ok(())
}