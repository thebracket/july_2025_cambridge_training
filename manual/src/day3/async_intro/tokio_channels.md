# Tokio - Channels

Tokio provides quite a few different channel types for inter-task communication. The most commonly used are `tokio::sync::mpsc` (multi-producer, single-consumer) and `tokio::sync::oneshot` (single-use, one sender and one receiver). There are also `broadcast` channels (one sender, multiple receivers) and `watch` channels (for broadcasting state changes). Additionally, the `async-channel` crate provides multi-producer, multi-consumer channels (as does `flume`).

## MPSC Channels

MPSC channels work very similarly to the standard library channels, but are async-aware. You can send messages from multiple tasks, and receive them in a single task.

```rust
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100); // buffer size - ALWAYS bounded!

    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}
```

Notice how that's almost identical to the example we used for the standard library? Instead of spawning a thread, we spawn a future/tokio task with `tokio::spawn`. Instead of blocking on `recv`, we `await` it. Instead of blocking on `send`, we `await` it. If the channel is full, the sender will yield until there is space in the channel.

I added the "current_thread" flavor to the runtime so that this example can run in a single thread. Non-async couldn't do that!

## Oneshot Channels

Oneshot channels are used for sending a single value from one task to another. They are useful for scenarios where you only need to send a single message and don't need the overhead of a full channel.

Here's an example of how to use a oneshot channel:

```rust
use tokio::sync::oneshot;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("Hello").unwrap();
    });

    let msg = rx.await.unwrap();
    println!("Received: {}", msg);
}
```

In this example, we create a oneshot channel and spawn a task that sends a single message. The main task then awaits the message and prints it.

A more realistic use case for oneshot channels is when you want to get a result back from a spawned task:

```rust
use tokio::sync::oneshot;

async fn compute() -> i32 {
    42 // Some expensive computation
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        let result = compute().await;
        tx.send(result).unwrap();
    });

    let result = rx.await.unwrap();
    println!("Computed result: {}", result);
}
```

This gives you the beginnings of the actor model. Transmit commands and results via channels, and have a task that processes them. It's also very close to the early concept of Objects in Smalltalk - send messages to objects, and they respond.