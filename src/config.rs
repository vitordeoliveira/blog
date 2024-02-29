use std::env;

use axum::{middleware, response::Response, routing::get, Router};
use blog::controller::root;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::error::{Error, Result};

// Config env OK
// Config Tracing OK
// Config Database
// Config Routes

struct Routes {}
struct Database {}
struct Environment {
    database_url: String,
    rust_log: String,
    server_port: String,
}

pub struct Config {
    routes: Routes,
    database: Database,
    environment: Environment,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let env = Environment {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URl should be set"),
            rust_log: env::var("RUST_LOG").unwrap_or("debug".to_string()),
            server_port: env::var("SERVER_PORT").unwrap_or("8000".to_string()),
        };

        Self {
            environment: env,
            routes: Routes {},
            database: Database {},
        }
    }

    pub async fn setup(self) -> Result<(TcpListener, Router)> {
        tracing_subscriber::fmt()
            .with_env_filter(self.environment.rust_log)
            .init();

        // let controller = Controller::new().await?;
        // let router = controller.get_routes().await?;

        let assets_path = std::env::current_dir().unwrap();
        let router = Router::new() // `GET /` goes to `root`
            .route("/", get(root))
            .layer(middleware::map_response(response_mapper))
            .nest_service(
                "/assets",
                ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
            );

        async fn response_mapper(res: Response) -> Response {
            info!("Response mapper");
            res
        }

        let listener =
            tokio::net::TcpListener::bind(format!("127.0.0.1:{}", self.environment.server_port))
                .await?;

        tracing::info!(
            "router initialized, now listening on port {}",
            self.environment.server_port
        );

        Ok((listener, router))
    }
}
