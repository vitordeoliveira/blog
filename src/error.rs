use std::{fmt::Debug, io};

use anyhow::Result;
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::error;

#[derive(Error)]
pub enum ServerError {
    #[error("Internal Server Error: {0}")]
    InternalServer(String),

    #[error("Error occur on metadata extraction of {0}")]
    YamlConvertionError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("PageNotFound: {0} not found")]
    PageNotFound(String),

    #[error("PageUnavailable: {0} has some error")]
    PageUnavailable(String),

    #[error("PageUnavailable: {0} has some error")]
    ConfigurationError(String),

    #[error("IOError: {0}")]
    IOError(#[from] io::Error),
    // #[error(transparent)]
    // Undefined(#[from] anyhow::Error),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error("sqlite error: {0}")]
    DBError(#[from] rusqlite::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        error!("Error {:?}", self);
        match self {
            ServerError::InternalServer(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
            ServerError::PageNotFound(_) => (StatusCode::NOT_FOUND, "Page not found"),
            ServerError::PageUnavailable(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Page unavailable in the moment",
            ),
            ServerError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            _ => (StatusCode::BAD_REQUEST, "Undefined error"),
        }
        .into_response()
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
