# Qiuck Workshop: Async Sleep and Print

> This is a quick workshop to get you familiar with writing async code in Rust using the Tokio runtime. It is intentionally simple, but covers the basics of async functions, sleeping, and joining multiple tasks.

Start a new binary project called `async_sleep_and_print` in your workspace. Add `tokio` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Write an async function that sleeps for a number of seconds (using `tokio::time::sleep`, the number of seconds should be passed as an argument), then prints a message indicating it has finished sleeping. 

In your `main` function, use the `#[tokio::main]` attribute to set up the Tokio runtime, and call your async sleep-and-print function multiple times with different sleep durations. Use `tokio::join!` to wait for all of them to complete.