use blog::{config::Config, error::Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    let (listener, router) = config.setup().await?;

    // Run Server
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
