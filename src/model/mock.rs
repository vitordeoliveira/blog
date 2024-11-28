use anyhow::Result;
use uuid::Uuid;

use super::User;

pub trait MockUser {
    fn new_mock() -> Result<User> {
        Ok(User {
            id: Uuid::default(),
            api_key: Uuid::default(),
        })
    }
}

impl MockUser for User {}
