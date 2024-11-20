use crate::error::ServerError;
use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PostInfo {
    pub id: String,
    pub views: u16,
}

pub trait SqliteOperations {
    fn find_or_create_post(sqlite_conn: &Connection, title: &str) -> Result<PostInfo, ServerError>;
    fn increment_views(sqlite_conn: &Connection, title: &str) -> Result<(), ServerError>;
}
