use chrono::{DateTime, Utc};
use std::fmt;

/// A type-safe wrapper around the database ID for a Mantra
/// This prevents accidentally mixing up IDs with other integers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MantraId(i64);

impl MantraId{
    /// Create a new MantraId
    pub fn new(id: i64) -> Self {
        MantraId(id)
    }

    /// Get the inner value
    pub fn value(&self) -> i64 {
        self.0
    }
}

// Implement Display for pretty printing
impl fmt::Display for MantraId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MantraId({})", self.0)
    }
}

/// Represents a mantra that can be sent to users
#[derive(Debug, Clone, PartialEq)]
pub struct Mantra {
    /// Unique identifier
    pub id: MantraId,

    /// The text content of the mantra
    pub text: String,

    /// Optional category for organising mantras
    pub category: Option<String>,

    /// When this mantra was created
    pub created_at: DateTime<Utc>,
}

impl Mantra {
    /// Create a new Mantra
    pub fn new(id: MantraId, text: String, category: Option<String>, created_at: DateTime<Utc>) -> Self {
        Mantra {
            id, 
            text,
            category,
            created_at,
        }
    }

    /// Create a mantra with the current timestamp
    pub fn new_now(id: MantraId, text: String, category: Option<String>) -> Self {
        Mantra {
            id, 
            text,
            category,
            created_at: Utc::now()
        }
    }
}

// Implement Display for pretty printing
impl fmt::Display for Mantra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.category {
            Some(cat) => write!(f, "[{}] {}", cat, self.text),
            None => write!(f, "{}", self.text)
        }
    }
}