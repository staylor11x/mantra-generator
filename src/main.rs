use chrono::Utc;
use mantra_service::domain::{select_mantra, Mantra, MantraId};

fn main() {
    println!("Testing Mantra Selection Logic\n");
    
    // Create some test mantras
    let mantras = vec![
        Mantra::new_now(
            MantraId::new(1),
            "I am calm and centered.".to_string(),
            Some("Mindfulness".to_string()),
        ),
        Mantra::new_now(
            MantraId::new(2),
            "Today is full of possibilities.".to_string(),
            Some("Motivation".to_string()),
        ),
        Mantra::new_now(
            MantraId::new(3),
            "I choose peace over worry.".to_string(),
            Some("Peace".to_string()),
        ),
        Mantra::new_now(
            MantraId::new(4),
            "I am worthy of good things.".to_string(),
            Some("Self-Love".to_string()),
        ),
    ];
    
    println!("Available mantras: {}", mantras.len());
    for mantra in &mantras {
        println!("  - {}", mantra);
    }
    
    // Test 1: Select with empty history
    println!("\n--- Test 1: Select with empty history ---");
    let history = vec![];
    match select_mantra(&mantras, &history) {
        Ok(selected) => println!("Selected: {}", selected),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test 2: Select with some history
    println!("\n--- Test 2: Select with history [1, 2] ---");
    let history = vec![MantraId::new(1), MantraId::new(2)];
    match select_mantra(&mantras, &history) {
        Ok(selected) => println!("Selected: {} (should be 3 or 4)", selected),
        Err(e) => println!("Error: {}", e),
    }
    
    // Test 3: All mantras exhausted
    println!("\n--- Test 3: All mantras exhausted ---");
    let history = vec![
        MantraId::new(1),
        MantraId::new(2),
        MantraId::new(3),
        MantraId::new(4),
    ];
    match select_mantra(&mantras, &history) {
        Ok(selected) => println!("Selected: {}", selected),
        Err(e) => println!("Error: {} ✓", e),
    }
    
    // Test 4: No mantras available
    println!("\n--- Test 4: No mantras available ---");
    let empty: Vec<Mantra> = vec![];
    match select_mantra(&empty, &[]) {
        Ok(selected) => println!("Selected: {}", selected),
        Err(e) => println!("Error: {} ✓", e),
    }
    
    // Test 5: Multiple random selections (see randomness)
    println!("\n--- Test 5: Multiple random selections ---");
    for i in 1..=5 {
        if let Ok(selected) = select_mantra(&mantras, &[]) {
            println!("  Selection {}: {}", i, selected.id);
        }
    }
}