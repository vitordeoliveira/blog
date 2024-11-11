use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use rusqlite::Connection;

#[derive(Clone, Debug)]
pub struct AppState {
    pub sqlite_conn: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(sqlite_path: &str) -> Result<Self> {
        let sqlite_conn = Arc::new(Mutex::new(
            Connection::open(sqlite_path).context("sqlite connection error")?,
        ));
        // sqlx::migrate!("db/migrations").run(&sqlite_pool).await?;
        Ok(Self { sqlite_conn })
    }
}
