use std::{env, sync::OnceLock};

use dotenv::dotenv;

use crate::error::{Error, Result};

pub struct Environment {
    pub rust_log: String,
    pub server_port: String,
    pub server_host: String,
    pub firestore_project_id: String,
}

impl Environment {
    fn load_from_env() -> Result<Self> {
        Ok(Self {
            server_port: get_env("SERVER_PORT")?,
            rust_log: get_env("RUST_LOG")?,
            server_host: get_env("SERVER_HOST")?,
            firestore_project_id: get_env("FIRESTORE_PROJECT_ID")?,
        })
    }
}

pub fn environment() -> &'static Environment {
    dotenv().ok();
    static INSTANCE: OnceLock<Environment> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Environment::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - CAUSE: {ex}"))
    })
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::InternalServer(format!("Env: {name} not found").to_string()))
}
