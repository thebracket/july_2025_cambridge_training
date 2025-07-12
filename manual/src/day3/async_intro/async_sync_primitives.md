# Async Synchronization Primitives

Holding onto Mutexes and similar outside of clear scope is a bad idea in general. In async code, it can be absolutely disastrous, because if you hold a lock across an `.await` boundary, you can deadlock the entire executor.

For this reason, the standard library does not provide async-aware synchronization primitives. Instead, we can use the ones provided by async runtimes like Tokio.

> Atomics are just fine in async code!

## Tokio Mutex

Tokio provides an async-aware `Mutex` that works similarly to the standard library one, but is safe to use across `.await` boundaries.

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            for _ in 0..1000 {
                let mut num = counter.lock().await;
                *num += 1;
            }
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("Final count: {}", *counter.lock().await);
}
```

Notice how this is basically identical to the standard library example, except we `await` the lock acquisition instead of using blocking. You can run it single-threaded or multi-threaded.