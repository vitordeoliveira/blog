use std::fs;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{self, Error, Result, ResultPath},
    model::{Markdown, MarkdownMetadata},
};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    metadata: MarkdownMetadata,
    content: String,
}

pub fn index() -> Result<impl IntoResponse> {
    // Get all blogpost names
    // extract all MarkdownMetadata
    // Append each metadata to a Vec
    // Return a htmx of all list for the entry page
    Ok((StatusCode::OK, Html("ok")))
}

pub fn show(Markdown { metadata, content }: Markdown) -> Result<impl IntoResponse> {
    let root = BlogTemplate { metadata, content };

    let html = match root.render() {
        Ok(html) => html,
        Err(_) => {
            return Err(error::Error::InternalServerError(
                "Error on render".to_string(),
            ))
        }
    };

    Ok((StatusCode::OK, Html(html)))
}
