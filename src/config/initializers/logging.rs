use crate::config::environment;

pub fn execute() {
    tracing_subscriber::fmt()
        .with_env_filter(environment().rust_log.clone())
        .with_thread_ids(true)
        .init();
}
