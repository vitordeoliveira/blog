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
        let mut sqlite_conn = Connection::open(sqlite_path).context("sqlite connection error")?;

        migrations::runner().run(&mut sqlite_conn).unwrap();

        let sqlite_conn = Arc::new(Mutex::new(sqlite_conn));

        Ok(Self { sqlite_conn })
    }
}
