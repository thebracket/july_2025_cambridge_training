# Demo: Channel Benchmark

Channels that aren't full (you can always send to a channel that isn't full) are very fast. But how fast? Let's benchmark it!

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..100_000 {
            tx.send(i).unwrap();
        }
    });

    let start = Instant::now();
    for _ in 0..100_000 {
        rx.recv().unwrap();
    }
    let duration = start.elapsed();

    println!("Time taken: {:?}", duration);
}
```

When I ran this on the Rust playground earlier, 100,000 messages took about 14ms. That's in debug mode.

> The code is in `code/day2/channel_bench` if you'd like to try it yourself.

On my office workstation I score about 9ms in debug mode, and 2.5 ms in release mode. Channels are unlikely to be your bottleneck!