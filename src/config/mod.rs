pub mod database;
mod environment;
mod initializers;

use self::environment::Environment;
use crate::{
    config::environment::environment,
    controller::{blog::Blog, home},
    error::Result,
};
use axum::{middleware, response::Response, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

struct Routes {
    blog: axum::Router,
    // blog_api: axum::Router,
}

pub struct Config {
    routes: Routes,
    environment: &'static Environment,
}

impl Config {
    pub fn new() -> Result<Self> {
        Ok(Self {
            environment: environment(),
            routes: Routes {
                blog: Blog::new().routes,
                // blog_api: Blog::new().routes_api,
            },
        })
    }

    pub async fn setup(self) -> Result<(TcpListener, Router)> {
        initializers::logging::execute();

        let assets_path = std::env::current_dir().unwrap();
        let router = Router::new() // `GET /` goes to `root`
            .route("/", get(home))
            .nest("/blog", self.routes.blog)
            // .nest("/api/v1/blog", self.routes.blog_api)
            .layer(middleware::map_response(response_mapper))
            .route_service("/sitemap.xml", ServeFile::new("sitemap.xml"))
            .nest_service(
                "/assets",
                ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
            );

        async fn response_mapper(res: Response) -> Response {
            res
        }

        let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            self.environment.server_host, self.environment.server_port
        ))
        .await?;

        tracing::info!(
            "router initialized, now listening on port {}",
            self.environment.server_port
        );

        Ok((listener, router))
    }
}
