# The Blocking Problem

If an async task blocks the thread it is running on, it blocks the entire runtime. Tokio in multi-threaded mode mitigates this with work-stealing, but it is still a problem.

First of all, *never* do this:

```rust
use std::thread::sleep;
use std::time::Duration;

async fn my_async_function() {
    println!("Starting blocking sleep...");
    sleep(Duration::from_secs(5)); // BAD! Blocks the entire thread!
    println!("Finished blocking sleep!");
}

#[tokio::main]
async fn main() {
    my_async_function().await;
}
```

The *entire* Tokio runtime is blocked for 5 seconds while `my_async_function` sleeps. This is a common mistake for people new to async programming.

## Heavy Calculations and Yielding

If you have a CPU-bound task that takes a long time, you should consider running it in a separate thread (using `tokio::task::spawn_blocking`), or breaking it up into smaller chunks and yielding periodically with `tokio::task::yield_now().await`.

```rust
use tokio::task;
use tokio::time::{sleep, Duration};
use std::time::Instant;

async fn heavy_computation() {
    let start = Instant::now();
    for i in 0..10 {
        // Simulate a chunk of work
        println!("Working on chunk {}", i);
        sleep(Duration::from_millis(500)).await; // Simulate work
        task::yield_now().await; // Yield to allow other tasks to run
    }
    println!("Heavy computation done in {:?}", start.elapsed());
}

#[tokio::main]
async fn main() {
    heavy_computation().await;
}
```

## spawn_blocking

If you have a blocking operation (like file I/O, database access, or heavy computation) that cannot be made async, use `tokio::task::spawn_blocking` to run it in a separate thread pool dedicated to blocking tasks:

```rust
use tokio::task;
use std::thread::sleep;
use std::time::Duration;

async fn blocking_task() {
    task::spawn_blocking(|| {
        println!("Starting blocking operation...");
        sleep(Duration::from_secs(5)); // This is OK, runs in a separate thread
        println!("Finished blocking operation!");
    }).await.unwrap();
}

#[tokio::main]
async fn main() {
    blocking_task().await;
}
```