use std::{fmt::Debug, io};

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error)]
pub enum Error {
    #[error("Internal Server Error {0}")]
    InternalServer(String),

    #[error("Page {0} not found")]
    PageNotFound(String),

    #[error("Page {0} has some error")]
    PageUnavailable(String),

    #[error("data store disconnected : {0}")]
    Disconnect(#[from] io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        error!("{self}");
        let response = match self {
            Error::InternalServer(_) => "INTERNAL_SERVER_ERROR".to_string(),
            Error::PageNotFound(_) => "Page not found".to_string(),
            Error::Disconnect(_) => "INTERNAL_SERVER_ERROR".to_string(),
            Error::PageUnavailable(_) => "Page unavailable in the moment".to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Html(response)).into_response()

        // Create a placeholder Axum response
        // StatusCode::INTERNAL_SERVER_ERROR.into_response()

        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
        // Insert the Error into the response.
        // response.extensions_mut().insert(self);
    }
}

pub struct ResultPath<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ResultPath<T>
where
    T: Send + Sync + Debug + DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(_) => Err(Error::PageNotFound("page not found".to_string())),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // error!("{self}");
        writeln!(f, "{}", self)?;
        Ok(())
    }
}
