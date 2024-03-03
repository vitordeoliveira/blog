use std::fs;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{self, Error, Result, ResultPath},
    model::MarkdownMetadata,
};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    description: String,
    tags: Vec<String>,
    similar_posts: Vec<String>,
    date: String,
    markdown_html: String,
}

pub fn index() -> Result<impl IntoResponse> {
    // Get all blogpost names
    // extract all MarkdownMetadata
    // Append each metadata to a Vec
    // Return a htmx of all list for the entry page
    Ok((StatusCode::OK, Html("ok")))
}

pub fn show(
    markdown_html: String,
    MarkdownMetadata {
        title,
        description,
        tags,
        similar_posts,
        date,
    }: MarkdownMetadata,
) -> Result<impl IntoResponse> {
    let root = BlogTemplate {
        title,
        description,
        tags,
        similar_posts,
        date,
        markdown_html,
    };

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
