use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::{self, Result},
    model::MarkdownMetadata,
};

pub mod blog;
pub mod home;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {
    title: String,
    posts: Vec<MarkdownMetadata>,
}

pub fn homepage(posts: Vec<MarkdownMetadata>) -> Result<impl IntoResponse> {
    let root = RootTemplate {
        title: "vitor.ws".to_string(),
        posts,
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(err) => return Err(error::Error::InternalServer(err.to_string())),
    };

    Ok((StatusCode::OK, Html(html)))
}
