use std::str::FromStr;

use anyhow::Result;
use axum::{
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

use tower_http::services::{ServeDir, ServeFile};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    controller::{
        blog::{
            load_markdown_content_api, load_markdown_similar_posts_api, markdown_list_api, show,
        },
        home,
    },
    error::ServerError,
    AppState, Markdown, SqliteOperations,
};

#[instrument]
// TODO: TEST
async fn api_key_auth_middleware(
    state: State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ServerError> {
    let api_key = request.headers().get("api_key");
    if let Some(api_key) = api_key {
        let sqlite_conn = state.get_connection()?;

        let api_key = Uuid::from_str(api_key.to_str().unwrap())?;
        let user = Markdown::get_user_from_api_key(&sqlite_conn, &api_key)?;

        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        Err(ServerError::Unauthorized)
    }
}

pub async fn new_app(app_state: AppState, assets_path: &str) -> Result<axum::Router> {
    let blog_view = Router::new().route("/:id", get(show));

    let blog_api = Router::new()
        .route("/v1/blog", get(markdown_list_api))
        .route("/v1/blog/:id", get(load_markdown_content_api))
        .route(
            "/v1/blog/:id/similar-posts",
            get(load_markdown_similar_posts_api),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            api_key_auth_middleware,
        ));

    let api = Router::new().nest("/api", blog_api);

    let router = Router::new() // `GET /` goes to `root`
        .route("/", get(home))
        .nest("/blog", blog_view)
        .merge(api)
        .with_state(app_state)
        .route_service("/sitemap.xml", ServeFile::new("sitemap.xml"))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path)));

    Ok(router)
}

#[cfg(test)]
mod tests {
    // #[tokio::test]
    // async fn api_key_auth_middleware_should_receive_every_string() {
    //     // api_key_auth_middleware(state, request, next)
    // }
}
