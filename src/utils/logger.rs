use tracing_subscriber::EnvFilter;

/// Sets up tracing with env-filter. Defaults to "velocity=info" if
/// RUST_LOG isn't set.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("velocity=info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}
