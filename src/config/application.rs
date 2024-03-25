pub fn execute() {
    tracing_subscriber::fmt()
        .with_env_filter(self.environment.rust_log.clone())
        .init();
}
