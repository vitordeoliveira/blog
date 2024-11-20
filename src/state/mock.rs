use anyhow::{Context, Result};
use rusqlite::Connection;

use super::{migrations, AppState};

pub trait MockAppState {
    fn new_mock(mock_sqlite_path: &str) -> Result<AppState> {
        let mut sqlite_conn = Connection::open(mock_sqlite_path)
            .map_err(|e| anyhow::anyhow!("sqlite connection error: {}", e))?;

        migrations::runner()
            .run(&mut sqlite_conn)
            .context("migration error")?;

        Ok(AppState {
            sqlite_path: mock_sqlite_path.to_string(),
            rust_env: "test".to_string(),
        })
    }
}

impl MockAppState for AppState {}