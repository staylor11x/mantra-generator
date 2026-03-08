use crate::domain::{Mantra, MantraId};
use rand::prelude::*;
use std::collections::HashSet;
use thiserror::Error;

/// Errors that can occur during mantra selection
#[derive(Error, Debug)]
pub enum SelectionError {
    #[error("No mantras available to select from")]
    NoMantrasAvailable,

    #[error("All {0} mantras have been sent recently")]
    AllMantrasExhausted(usize)
}

/// Selects a random mantra that hasn't been sent recently
/// 
/// # Arguments
/// * `available` - Slice of all available mantras
/// * `history` - Slice of mantra IDs that were recently sent
/// 
/// # Returns
/// * `Ok(&Mantra)` - A randomly selected mantra not in history
/// * `Err(SelectionError)` - If no mantras available or all exhausted
/// 
/// # Examples
/// ```
/// use mantra_service::domain::{Mantra, MantraId, select_mantra};
/// 
/// # // Hidden setup code 
/// let mantras = vec![
///     Mantra::new_now(MantraId::new(1), "Test".to_string(), None),
/// ];
/// let history = vec![];
/// let selected = select_mantra(&mantras, &history).unwrap();
/// # assert!(selected.id == MantraId::new(1))
/// ```
pub fn select_mantra<'a>(
    available: &'a [Mantra],
    history: &[MantraId],
) -> Result<&'a Mantra, SelectionError> {
    if available.is_empty() {
        return Err(SelectionError::NoMantrasAvailable);
    }

    // Convert history to HashSet for 0(1) lookup instead of O(n) >.<
    let sent_ids: HashSet<MantraId> = history.iter().copied().collect();

    // Filter mantras that haven't been sent
    let unsent: Vec<&Mantra> = available
        .iter()
        .filter(|mantra| !sent_ids.contains(&mantra.id))
        .collect();

    // Check if all mantras have been sent
    if unsent.is_empty() {
        return Err(SelectionError::AllMantrasExhausted(available.len()));
    }

    // Select a random mantra from unsent ones
    let mut rng = rand::rng();
    unsent
        .choose(&mut rng)
        .copied()   // Convert &&Mantra to &Mantra
        .ok_or(SelectionError::NoMantrasAvailable)
}

#[cfg(test)]
mod tests{
    use super::*;   // import everything from the parent module

    fn create_test_mantras() -> Vec<Mantra> {
        vec![
            Mantra::new_now(
                MantraId::new(1), 
                "I am calm and centered".to_string(), 
                Some("Mindfulness".to_string()),
            ),
            Mantra::new_now(
                MantraId::new(2),
                "Today is full of possibilities".to_string(),
                Some("Motivation".to_string()),
            ),
            Mantra::new_now(
                MantraId::new(3), 
                "I choose peace over worry".to_string(),
                Some("Peace".to_string()),
            ),
            Mantra::new_now(
                MantraId::new(4),
                "I am worthy of good things".to_string(),
                Some("Self-Love".to_string()),
            ),
        ]
    }

    #[test]
    fn test_select_with_empty_history() {
        // Arrange
        let mantras = create_test_mantras();
        let history = vec![];
    
        // Act
        let result = select_mantra(&mantras, &history);
    
        // Assert
        assert!(result.is_ok(), "Should select a mantra when history is empty");
    
        let selected = result.unwrap();
        assert!(
            mantras.iter().any(|m| m.id == selected.id),
            "Selected mantra should be from available mantras"
        );
    }

    #[test]
    fn test_selection_excludes_history() {
        // Arrange
        let mantras = create_test_mantras();
        let history = vec!{MantraId::new(1), MantraId::new(2)};

        // Act
        let result = select_mantra(&mantras, &history);

        // Assert
        assert!(result.is_ok());
        
        let selected = result.unwrap();
        // should only select from IDs 3 or 4 as they are not in history
        assert!(
            selected.id == MantraId::new(3) || selected.id == MantraId::new(4),
            "Selected mantra should not be in history, Got: {:?}",
            selected.id
        );
    }

    #[test]
    fn test_error_when_all_mantras_exhausted(){
        // Arrange
        let mantras = create_test_mantras();
        let history = vec![
            MantraId::new(1),
            MantraId::new(2),
            MantraId::new(3),
            MantraId::new(4),
        ];

        // Act 
        let result = select_mantra(&mantras, &history);

        // Assert
        assert!(result.is_err(), "Should return error when all mantras exhausted");

        // Check specific error variant
        match result {
            Err(SelectionError::AllMantrasExhausted(count)) => {
                assert_eq!(count, 4, "Should report correct number of exhausted mantras");
            }
        _ => panic!("Expected AllMantrasExhausted error"),
        }
    }

    #[test]
    fn test_error_when_no_mantras_available() {
        let mantras: Vec<Mantra> = vec![];
        let history = vec![];

        let result = select_mantra(&mantras, &history);

        assert!(matches!(result, Err(SelectionError::NoMantrasAvailable)));
    }

    #[test]
    fn test_single_mantra_not_in_history() {
        // Arrange
        let mantras = vec![
            Mantra::new_now(
                MantraId::new(1),
                "Only mantra.".to_string(),
                None,
            ),
        ];
        let history = vec![];

        // Act
        let result = select_mantra(&mantras, &history);

        // Assert
        assert!(result.is_ok());
        let selected = result.unwrap();
        assert_eq!(selected.id, MantraId::new(1));
    }

    #[test]
    fn test_single_mantra_in_history(){
        // Arrange
        let mantras = vec![
            Mantra::new_now(
                MantraId::new(1), 
                "Only mantra".to_string(),
                None,
            )
        ];
        let history = vec![MantraId::new(1)];

        // Act
        let result = select_mantra(&mantras, &history);

        // Assert
        assert!(result.is_err());
        assert!(matches!(result, Err(SelectionError::AllMantrasExhausted(1))));
    }

    #[test]
    fn test_randomness_distribution(){
        // This test verifies that selection is actually random!
        let mantras = create_test_mantras();
        let history = vec![];

        let mut selected_ids = std::collections::HashSet::new();

        // Run selection 20 times
        for _ in 0..20 {
            if let Ok(selected) = select_mantra(&mantras, &history) {
                selected_ids.insert(selected.id);
            }
        }

        // With 20 selections from 4 mantras, we should see multiple different ones
        // (This could theoretically fail due to its own randomness, but this is unlikely)
        assert!(
            selected_ids.len() > 1,
            "Should select different mantras over multiple runs. Got: {:?}",
            selected_ids
        );
    }
}
