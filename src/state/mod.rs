use anyhow::{Context, Result};
use rusqlite::Connection;
refinery::embed_migrations!("migrations");

#[cfg(test)]
pub mod mock;

#[derive(Clone, Debug)]
pub struct AppState {
    pub sqlite_path: String,
    pub rust_env: String,
}

impl AppState {
    pub fn new(sqlite_path: &str, rust_env: &str) -> Result<Self> {
        let mut sqlite_conn = Connection::open(sqlite_path)
            .map_err(|e| anyhow::anyhow!("sqlite connection error: {}", e))?;

        migrations::runner()
            .run(&mut sqlite_conn)
            .context("migration error")?;

        Ok(Self {
            sqlite_path: sqlite_path.to_string(),
            rust_env: rust_env.to_string(),
        })
    }

    pub fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.sqlite_path)
            .map_err(|e| anyhow::anyhow!("sqlite connection error: {}", e))
    }
}
