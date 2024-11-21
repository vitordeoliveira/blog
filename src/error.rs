use std::{fmt::Debug, io};

use anyhow::Result;
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::error;

#[derive(Error)]
pub enum ServerError {
    #[error("Internal Server Error {0}")]
    InternalServer(String),

    #[error("Page {0} not found")]
    PageNotFound(String),

    #[error("Page {0} has some error")]
    PageUnavailable(String),

    #[error("data store disconnected : {0}")]
    Disconnect(#[from] io::Error),

    #[error(transparent)]
    Undefined(#[from] anyhow::Error),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error(transparent)]
    DBError(#[from] rusqlite::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        error!("{self}");
        let response = match self {
            ServerError::InternalServer(_) => "INTERNAL_SERVER_ERROR".to_string(),
            ServerError::PageNotFound(_) => "Page not found".to_string(),
            ServerError::Disconnect(_) => "INTERNAL_SERVER_ERROR".to_string(),
            ServerError::PageUnavailable(_) => "Page unavailable in the moment".to_string(),
            ServerError::Undefined(_error) => "Undefined error".to_string(),
            ServerError::DBError(_error) => "Undefined error".to_string(),
            ServerError::UuidError(_error) => "Undefined error".to_string(),
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
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, ServerError> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(_) => Err(ServerError::PageNotFound("page not found".to_string())),
        }
    }
}

impl Debug for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // error!("{self}");
        writeln!(f, "{}", self)?;
        Ok(())
    }
}
