fn main() {
    // Setup tracing subscriber for JSON output
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    tracing::info!("Starting application");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");
    tracing::debug!("This is a debug message");
}
