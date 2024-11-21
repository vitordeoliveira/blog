use std::str::FromStr;

use crate::error::ServerError;
use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tracing::instrument;
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
    #[instrument]
    // TODO: TEST
    fn find_or_create_post(sqlite_conn: &Connection, title: &str) -> Result<PostInfo, ServerError> {
        let mut stmt = sqlite_conn.prepare_cached("SELECT id, views FROM posts WHERE id = ?1")?;

        let post = stmt.query_row([title], |row| {
            Ok(PostInfo {
                id: row.get(0)?,
                views: row.get(1)?,
            })
        });

        Ok(match post {
            Ok(post) => PostInfo {
                id: post.id,
                views: post.views,
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                sqlite_conn.execute("INSERT INTO posts (id) values (?1)", params![title])?;

                let post = stmt.query_row([title], |row| {
                    Ok(PostInfo {
                        id: row.get(0)?,
                        views: row.get(1)?,
                    })
                })?;

                PostInfo {
                    id: post.id,
                    views: post.views,
                }
            }
            Err(e) => return Err(ServerError::DBError(e)),
        })
    }

    #[instrument]
    // TODO: TEST
    fn increment_views(sqlite_conn: &Connection, title: &str) -> Result<(), ServerError> {
        sqlite_conn.execute(
            "UPDATE posts SET views = views + 1 WHERE id = ?1",
            params![title],
        )?;

        Ok(())
    }

    #[instrument]
    // TODO: TEST
    fn get_user_from_api_key(
        sqlite_conn: &Connection,
        api_key: &Uuid,
    ) -> Result<User, ServerError> {
        let mut stmt = sqlite_conn.prepare_cached("SELECT * FROM users WHERE api_key = ?1")?;

        let result = match stmt.query_row([api_key.to_string()], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }) {
            Ok(v) => v,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(ServerError::Unauthorized),
            Err(e) => return Err(ServerError::DBError(e)),
        };

        Ok(User {
            id: Uuid::from_str(&result.0)?,
            api_key: Uuid::from_str(&result.1)?,
        })
    }
}
