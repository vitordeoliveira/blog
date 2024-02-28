use std::{fmt::Debug, io};

use axum::{http::StatusCode, response::IntoResponse};
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

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        // Create a placeholder Axum response
        StatusCode::INTERNAL_SERVER_ERROR.into_response()

        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
        // Insert the Error into the response.
        // response.extensions_mut().insert(self);
    }
}

impl Debug for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error!("{self}");
        // writeln!(f, "{}", self)?;
        Ok(())
    }
}
