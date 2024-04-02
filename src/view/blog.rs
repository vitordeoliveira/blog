use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{Error, Result},
    model::{Markdown, MarkdownMetadata, PostInfo},
};

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    metadata: MarkdownMetadata,
    content: String,
    similar_posts_metadata: Vec<(MarkdownMetadata, PostInfo)>,
}

pub fn show(
    Markdown { metadata, content }: Markdown,
    similar_posts_metadata: Vec<(MarkdownMetadata, PostInfo)>,
) -> Result<impl IntoResponse> {
    let root = BlogTemplate {
        metadata,
        content,
        similar_posts_metadata,
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(_) => return Err(Error::InternalServer("Error on render".to_string())),
    };

    Ok((StatusCode::OK, Html(html)))
}
