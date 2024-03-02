use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::error::{self, Result};

pub mod blog;
pub mod home;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {
    title: String,
}

pub fn root() -> Result<impl IntoResponse> {
    let root = RootTemplate {
        title: "vitor.ws".to_string(),
    };

    let html = match root.render() {
        Ok(html) => html,
        Err(err) => return Err(error::Error::InternalServerError(err.to_string())),
    };

    Ok((StatusCode::OK, Html(html)))
}
