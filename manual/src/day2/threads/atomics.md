# Atomics

Fortunately, for a shared counter - an atomic variable is all you need. Atomic variables are thread-safe, and they allow you to perform operations on them without needing locks. Rust provides atomic types in the `std::sync::atomic` module.

```rust
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let counter = AtomicI32::new(0);

    std::thread::scope(|s| {
        for _ in 0..3 {
            s.spawn(|| {
                for _ in 0..100_000 {
                    // Atomically increment the counter
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    println!("Counter: {}", counter.load(Ordering::Relaxed));
}
```

Note that for "ordering" the Rust manual is uninentionally hilarious, referring you to the C++ standard!

> Buy Mara Bos' excellent Rust Atomics and Locks. You can also read it online [here](https://marabos.nl/atomics/) - but Mara's great and deserves some book sales!
