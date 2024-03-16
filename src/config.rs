use std::{env, sync::OnceLock};

use axum::{middleware, response::Response, routing::get, Router};

use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    controller::{blog::Blog, home},
    error::{Error, Result},
};

struct Routes {
    blog: axum::Router,
}

pub struct Environment {
    pub rust_log: String,
    pub server_port: String,
    server_host: String,
}

impl Environment {
    fn load_from_env() -> Result<Self> {
        Ok(Self {
            server_port: get_env("SERVER_PORT")?,
            rust_log: get_env("RUST_LOG")?,
            server_host: get_env("SERVER_HOST")?,
        })
    }
}

pub fn environment() -> &'static Environment {
    dotenv().ok();
    static INSTANCE: OnceLock<Environment> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Environment::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - CAUSE: {ex}"))
    })
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::InternalServer(format!("Env: {name} not found").to_string()))
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
            },
        })
    }

    pub async fn setup(self) -> Result<(TcpListener, Router)> {
        tracing_subscriber::fmt()
            .with_env_filter(self.environment.rust_log.clone())
            .init();

        let assets_path = std::env::current_dir().unwrap();
        let router = Router::new() // `GET /` goes to `root`
            .route("/", get(home))
            .nest("/blog", self.routes.blog)
            .layer(middleware::map_response(response_mapper))
            .route_service("/sitemap", ServeFile::new("sitemap.xml"))
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
