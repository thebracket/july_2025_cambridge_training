use std::time::Duration;

use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

/// Set up tracing logging for the application.
pub fn set_console_logging() {
    // install a global collector configured based on RUST_LOG env var.
    let subscriber = tracing_subscriber::fmt()
        // Configure the subscriber to read the RUST_LOG environment variable
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(false)
        // Include per-span timings
        .with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tracing::instrument]
fn my_function() {
    std::thread::sleep(Duration::from_millis(100));
}

fn main() {
    set_console_logging();
    tracing::info!("Starting application");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");
    tracing::debug!("This is a debug message");
    my_function();
}
