use std::error::Error;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

use crate::models::Url;

pub struct SqliteDb {
    pool: &'static Pool<SqliteConnectionManager>,
}

impl SqliteDb {
    pub fn default(pool: &'static Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    pub fn init_table(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.pool.get().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS short_urls (
              id          INTEGER PRIMARY KEY AUTOINCREMENT,
              short_code  TEXT UNIQUE NOT NULL,
              long_url    TEXT NOT NULL,
              created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
          )",
            [], // no params
        )?;

        println!("✅ short_urls table ready");
        Ok(())
    }

    pub fn create_short_url(&self, short_code: &str, long_url: &str) -> Result<(), Box<dyn Error>> {
        println!("Creating short URL: {} -> {}", short_code, long_url);
        let conn = self.pool.get().unwrap();
        conn.execute(
            "INSERT INTO short_urls (short_code, long_url) VALUES (?, ?)",
            [short_code, long_url],
        )?;
        Ok(())
    }

    pub fn get_long_url(&self, short_code: &str) -> rusqlite::Result<Option<Url>> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT long_url FROM short_urls WHERE short_code = ?")?;
        let mut rows = stmt.query(params![short_code])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Url::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    pub fn remove_rows_created_in_last_day(&self) -> rusqlite::Result<()> {
        let conn = self.pool.get().unwrap();
        let mut stmt =
            conn.prepare("DELETE FROM short_urls WHERE created_at < datetime('now', '-1 day')")?;
        stmt.execute(params![])?;
        Ok(())
    }
}
