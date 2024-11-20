use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    controller::{
        blog::{load_markdown_content_api, load_markdown_similar_posts_api, show},
        home,
    },
    AppState,
};

pub async fn new_app(app_state: AppState, assets_path: &str) -> Result<axum::Router> {
    let blog_view = Router::new().route("/:id", get(show));

    let blog_api = Router::new()
        .route("/v1/blog/:id", get(load_markdown_content_api))
        .route(
            "/v1/blog/:id/similar-posts",
            get(load_markdown_similar_posts_api),
        );

    let api = Router::new().nest("/api", blog_api);

    let router = Router::new() // `GET /` goes to `root`
        .route("/", get(home))
        .nest("/blog", blog_view)
        .merge(api)
        // .nest("/api/v1/blog", self.routes.blog_api)
        .with_state(app_state)
        .route_service("/sitemap.xml", ServeFile::new("sitemap.xml"))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path)));

    Ok(router)
}
