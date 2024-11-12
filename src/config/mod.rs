mod environment;

pub struct Config {}

impl Config {
    pub async fn logging(rust_log: &str) {
        tracing_subscriber::fmt()
            .with_env_filter(rust_log)
            .with_thread_ids(true)
            .init();
    }
}
