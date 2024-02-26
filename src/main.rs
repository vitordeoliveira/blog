use config::Config;

mod config;

fn main() -> Result<(), ()> {
    // Config env
    // Config Tracing
    // Config Database
    // Config Routes
    let config = Config::new();
    config.setup()?;

    // Run Server
    Ok(())
}
