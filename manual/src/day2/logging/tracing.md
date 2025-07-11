# Tracing & Tracing Subscriber

> For a long time, the `log` crate was the preferred way to do logging in Rust. It provides a simple API for logging messages at different levels (error, warn, info, debug, trace) and allows you to plug in different logging implementations. It has now largely been replaced by the `tracing` crate, which provides more powerful features like structured logging and spans.

The `tracing` crate is split into two parts:
- **Tracing**: The core library that provides the API for logging and spans.
- **Tracing Subscriber**: A library that provides a way to collect and process the logs and spans.

You can use any subscriber with `tracing` (lots exist), but the most common one is `tracing-subscriber`. It provides a way to collect and process the logs and spans, and it can be configured to output the logs in different formats (e.g., JSON, text).

## Simple Logging Example

> This example is found in `code/day2/tracing_simple`.

Our workspace includes:

```toml
[workspace.dependencies]
tracing = "0.1.41"
tracing-subscriber = {version = "0.3.19", features = ["fmt", "env-filter", "json"]}
```

In `Cargo.toml` we've imported the `tracing` and `tracing-subscriber` crates:

```toml
[dependencies]
tracing.workspace = true
tracing-subscriber.workspace = true
```

The program is very simple:

```rust
fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting application");
}
```

## A Slightly More Complex Example

> This example is found in `code/day2/tracing_simple2`.

Let's add some more details to what we're logging:

```rust
use tracing_subscriber::EnvFilter;

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
        //.with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() {
    set_console_logging();
    tracing::info!("Starting application");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");
    tracing::debug!("This is a debug message");
}
```

And now `cargo run` shows just the info, warn, and error messages. `RUST_LOG=debug cargo run` will also show the debug messages. We've added the code line on which the item was logged, and the file name.

## Tracing Spans

Tracing spans are a great way to time the execution of a block of code. They can be nested, and you can add fields to them to provide more context. It's very common to forward them to something like OpenTelemetry.

> This example is found in `code/day2/tracing_spans`.

Uncomment the line with `with_span_events(FmtSpan::CLOSE)` in the previous example to see how spans work. Now we can add a function and time its execution:

```rust
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
```

This gives output:

```
2025-07-11T18:19:40.425454Z  INFO day2/tracing_spans/src/main.rs:37: Starting application
2025-07-11T18:19:40.425482Z  WARN day2/tracing_spans/src/main.rs:38: This is a warning message
2025-07-11T18:19:40.425488Z ERROR day2/tracing_spans/src/main.rs:39: This is an error message
2025-07-11T18:19:40.525777Z  INFO my_function: day2/tracing_spans/src/main.rs:30: close time.busy=100ms time.idle=18.1µs
```

## Structured Logging

