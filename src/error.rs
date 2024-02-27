use std::{fmt::Debug, io};

use thiserror::Error;
use tracing::{error, info};
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("data store disconnected : {0}")]
    Disconnect(#[from] io::Error),
}

impl Debug for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error!("{self}");
        // writeln!(f, "{}", self)?;
        Ok(())
    }
}
