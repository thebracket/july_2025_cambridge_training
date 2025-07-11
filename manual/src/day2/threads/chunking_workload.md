# Chunking Workload

Vectors include a handy `chunks()` method that allows you to divide a vector into mostly-equal chunks (there's `chunks_exact`, too). You can use this to parallelize work across multiple threads:

```rust
use std::thread;

fn main() {
    let workload = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunks = workload.chunks(3); // 3 items each chunk

    thread::scope(|s| {
        for chunk in chunks {
            s.spawn(move || {
                let sum: i32 = chunk.iter().sum();
                println!("Sum of chunk {:?} is {}", chunk, sum);
            });
        }
    });
}
```