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
    config::blog::Blog,
    controller::{
        blog::{
            load_markdown_content_api, load_markdown_similar_posts_api, markdown_list_api, show,
        },
        home,
    },
    error::ServerError,
    model::User,
    state::{AppState, ConfigState},
    EnvState, Markdown, SqliteOperations,
};

// TODO: TEST
#[instrument(skip_all, fields(self.uri = %request.uri(), host = ?request.headers().get("host")))]
async fn mw_extract_blog_from_config_state(
    state: State<ConfigState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ServerError> {
    let user = request.extensions().get::<User>().unwrap().id.to_string();

    let blog_config: Blog = state
        .blog_config
        .blog
        .clone()
        .into_iter()
        .find(|blog| blog.user.clone() == user.clone())
        .unwrap();

    request.extensions_mut().insert(blog_config);

    Ok(next.run(request).await)
}

// TODO: TEST
#[instrument(skip_all, fields(self.uri = %request.uri(), host = ?request.headers().get("host")))]
async fn mw_extract_user_from_key(
    state: State<EnvState>,
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

pub async fn new_app(state: AppState) -> Result<axum::Router, ServerError> {
    let blog_view = Router::new().route("/:id", get(show));

    let blog_api = Router::new()
        .route("/v1/blog", get(markdown_list_api))
        .route("/v1/blog/:id", get(load_markdown_content_api))
        .route(
            "/v1/blog/:id/similar-posts",
            get(load_markdown_similar_posts_api),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_extract_blog_from_config_state,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_extract_user_from_key,
        ));

    let api = Router::new().nest("/api", blog_api);

    let router = Router::new() // `GET /` goes to `root`
        .route("/", get(home))
        .nest("/blog", blog_view)
        .merge(api)
        .with_state(state.clone())
        .route_service("/sitemap.xml", ServeFile::new("sitemap.xml"))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", state.config_state.clone().assets_path)),
        );

    Ok(router)
}

#[cfg(test)]
mod tests {
    // #[tokio::test]
    // async fn api_key_auth_middleware_should_receive_every_string() {
    //     // api_key_auth_middleware(state, request, next)
    // }
}
