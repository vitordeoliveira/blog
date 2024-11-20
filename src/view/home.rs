use anyhow::Result;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{self, ServerError},
    model::{MarkdownMetadata, PostInfo},
};

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {
    title: String,
    posts: Vec<(MarkdownMetadata, PostInfo)>,
}

pub fn homepage(
    posts: Vec<(MarkdownMetadata, PostInfo)>,
) -> Result<impl IntoResponse, ServerError> {
    let root = RootTemplate {
        title: "vitor.ws".to_string(),
        posts,
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(err) => return Err(error::ServerError::InternalServer(err.to_string())),
    };

    Ok((StatusCode::OK, Html(html)))
}
