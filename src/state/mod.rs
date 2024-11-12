use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use rusqlite::Connection;

refinery::embed_migrations!("migrations");

#[derive(Clone, Debug)]
pub struct AppState {
    pub sqlite_conn: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(sqlite_path: &str) -> Result<Self> {
        let sqlite_conn = Arc::new(Mutex::new(
            Connection::open(sqlite_path).context("sqlite connection error")?,
        ));

        if let Ok(mut conn) = sqlite_conn.lock() {
            migrations::runner()
                .run(&mut *conn)
                .context("migration error")?;
        }

        Ok(Self { sqlite_conn })
    }
}
