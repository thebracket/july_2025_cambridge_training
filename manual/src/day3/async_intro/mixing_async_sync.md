# Mixing Async and Syncronous Rust

Peanut butter and chocolate. Cats and dogs. Sync and async. None of those are as difficult to mix as you might think!

```rust
fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(5);

    // An async thread receiving messages
    std::thread::spawn(move || {
        // Spawn a current-thread Tokio runtime
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            while let Some(msg) = rx.recv().await {
                println!("Received in async: {}", msg);
            }
        });
    });

    // Sync sending messages
    for i in 0..10 {
        tx.blocking_send(i).unwrap(); // blocking_send is sync-friendly
    }

    // Give the async thread a moment to process messages before exiting
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

In this example, we create a Tokio MPSC channel, spawn a standard thread that runs a current-thread Tokio runtime to receive messages asynchronously, and then send messages from the main synchronous thread using `blocking_send`. This works with `oneshot`, `flume`, and other async-aware channels that provide sync-friendly sending methods.

It is a *very* common pattern to have a sync main thread that spawns an async runtime in a thread, and then communicates with it via channels. This is especially useful for integrating async code into existing sync applications.