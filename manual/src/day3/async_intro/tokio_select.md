# Tokio - Selecting Channels

The `select!` macro in Tokio allows you to wait on multiple asynchronous operations simultaneously, proceeding with whichever operation completes first. This is particularly useful when working with multiple channels or tasks where you want to react to the first available result.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(5);
    let (tx2, mut rx2) = mpsc::channel(5);

    tokio::spawn(async move {
        tx1.send("Message from channel 1").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("Message from channel 2").await.unwrap();
    });

    tokio::select! {
        Some(msg) = rx1.recv() => {
            println!("Received: {}", msg);
        }
        Some(msg) = rx2.recv() => {
            println!("Received: {}", msg);
        }
    }
}
```

In this case, we're not looping - so the other message is *simply lost*.

You can use `select!` to implement timeouts, by making one branch wait on a timer. A common pattern is to use `interval` to create a repeating timer:

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(5);
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    tokio::spawn(async move {
        loop {
            tx.send("Hello after 2 seconds").await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.1)).await;
        }
    });

    let mut tick_count = 0;
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                println!("Received: {}", msg);
            }
            _ = interval.tick() => {
                println!("Tick");
                tick_count += 1;
                if tick_count >= 5 {
                    println!("Exiting after five ticks.");
                    break;
                }
            }
        }
    }
}
```

> This example is in `code/day3/async_select` in the repository.
