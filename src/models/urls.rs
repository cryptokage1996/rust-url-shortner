use rusqlite::{Row, Result};

#[derive(Debug, Clone)]
pub struct Url {
    pub id: i64,
    pub short_code: String,
    pub long_url: String,
    pub created_at: String, // stored as TEXT in SQLite
}

impl Url {
    /// Helper: map from SQLite row → Url
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            short_code: row.get("short_code")?,
            long_url: row.get("long_url")?,
            created_at: row.get("created_at")?,
        })
    }
}