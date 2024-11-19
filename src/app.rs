use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    controller::{blog::show, home},
    AppState,
};

pub async fn new_app(app_state: AppState, assets_path: &str) -> Result<axum::Router> {
    let blog_routes = Router::new().route("/:id", get(show));

    let router = Router::new() // `GET /` goes to `root`
        .route("/", get(home))
        .nest("/blog", blog_routes)
        // .nest("/api/v1/blog", self.routes.blog_api)
        .with_state(app_state)
        .route_service("/sitemap.xml", ServeFile::new("sitemap.xml"))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path)));

    Ok(router)
}
