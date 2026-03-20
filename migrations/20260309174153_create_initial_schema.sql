-- Enable foreign key constraints (disabled by default in SQLite)
PRAGMA foreign_keys = ON;

-- Mantras table: stores all available mantras
CREATE TABLE IF NOT EXISTS mantras (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL,
    category TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Sent history table: tracks when mantras were sent
CREATE TABLE IF NOT EXISTS sent_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mantra_id INTEGER NOT NULL,
    sent_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (mantra_id) REFERENCES mantras(id) ON DELETE CASCADE
);

-- Index for faster queries on sent_at (for finding recent sends)
CREATE INDEX IF NOT EXISTS idx_sent_history_sent_at ON sent_history(sent_at);

-- Index for faster lookups by mantra_id
CREATE INDEX IF NOT EXISTS idx_sent_history_mantra_id ON sent_history(mantra_id);

-- Index on category for category-based filtering
CREATE INDEX IF NOT EXISTS idx_mantras_category ON mantras(category);
