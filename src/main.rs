use config::Config;

mod config;

async fn main() -> Result<(), ()> {
    // Config env
    // Config Tracing
    // Config Database
    // Config Routes
    let config = Config::new();
    let (listener, router) = config.setup().await?;

    // Run Server
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
