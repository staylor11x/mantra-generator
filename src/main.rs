// use chrono:: Utc;
use mantra_service::domain::{Mantra, MantraId};

fn main() {
    println!("Hello, Mantra Generator");

    // Create a MantraId
    let id = MantraId::new(1);
    println!("Created: {}", id);

    // Create a mantra with a category
    let mantra1 = Mantra::new_now(
        id,
        "Today I choose to be present in every moment.".to_string(),
        Some("Mindfulness".to_string()),
    );

    let mantra2 = Mantra::new_now(
        MantraId::new(2),
        "I am capable of achieving my goals.".to_string(),
        None,
    );

    println!("\nMantra 1: {}", mantra1);
    println!("\nMantra 2: {}", mantra2);

    let mantra3 = mantra1.clone();
    println!("\nAre mantra1 and mantra3 equal? {}", mantra1 == mantra3)
}
