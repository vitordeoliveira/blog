use axum::{response::IntoResponse, routing::get, Router};

use crate::{
    error::Result,
    view::{blog::blog_view, root as rootView},
};

pub async fn root_page() -> Result<impl IntoResponse> {
    rootView()
}

#[derive(Default)]
pub struct Blog {
    pub routes: axum::Router,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            routes: Router::new().route("/:id", get(blog_view)),
        }
    }
}
