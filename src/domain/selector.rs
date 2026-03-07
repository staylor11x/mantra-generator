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
/// let mantras = vec![/* ... */];
/// let history = vec![MantraId::new(1)];
/// let selected = select_mantra(&mantras, &history)?;
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