use std::str::FromStr;

use crate::error::ServerError;
use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PostInfo {
    pub id: String,
    pub views: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub api_key: Uuid,
}

pub trait SqliteOperations {
    fn find_or_create_post(sqlite_conn: &Connection, title: &str) -> Result<PostInfo, ServerError>;
    fn increment_views(sqlite_conn: &Connection, title: &str) -> Result<(), ServerError>;
    fn get_user_from_api_key(
        sqlite_conn: &Connection,
        api_key: &Uuid,
    ) -> Result<User, ServerError> {
        let mut stmt = sqlite_conn.prepare_cached("SELECT * FROM users WHERE api_key = ?1")?;

        let result = stmt.query_row([api_key.to_string()], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        Ok(User {
            id: Uuid::from_str(&result.0)?,
            api_key: Uuid::from_str(&result.1)?,
        })
    }
}
