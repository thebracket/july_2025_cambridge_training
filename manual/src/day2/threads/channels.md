# Channels

Channels are advertised as Go's superpower, but Rust (and plenty of other languages) have them too! Channels are a way to send messages between threads, allowing for safe communication without shared memory.

Rob Pike famously said "don't communicate by sharing memory; share memory by communicating". Channels are a great way to do this in Rust.

In threaded land, channels are a way to send messages between threads without needing locks.

## Basic Channels

```rust
use std::sync::mpsc; // mpsc = multiple producer, single consumer
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that sends messages
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap(); // Send a message
        }
    });

    // Receive messages in the main thread
    for received in rx {
        println!("Received: {}", received);
    }
}
```

Notice that when we run this  - the program stops cleanly! When a channel has no more senders (`tx` acts like an `Arc` - cheap to clone and reference counted), the receiver (`rx`) stops receiving messages.

You can send a lot more than just integers over channels. You can send any type that implements the `Send` trait, which includes most types in Rust. I'm particularly fond of sending `enum` types over channels as a "command pattern". Here's an example:

```rust
use std::sync::mpsc;
use std::thread;

enum Command {
    Increment,
    Decrement,
}
struct Counter {
    value: i32,
}
impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn execute(&mut self, command: Command) {
        match command {
            Command::Increment => self.value += 1,
            Command::Decrement => self.value -= 1,
        }
    }
}
fn main() {
    let (tx, rx) = mpsc::channel();
    let t1 = thread::spawn(move || {
        let mut counter = Counter::new();
        for command in rx {
            counter.execute(command);
        }
        println!("Final counter value: {}", counter.value);
    });
    tx.send(Command::Increment).unwrap();
    tx.send(Command::Decrement).unwrap();
    tx.send(Command::Increment).unwrap();
    drop(tx); // Close the channel
    t1.join().unwrap();
}
```

This is very common in Rust as an *actor model*. You sidestep locking by having a single thread own the data, and other threads send commands to it. This is a great way to avoid deadlocks and keep your code simple. It also scales well when you have multiple producers sending commands to a single consumer. On a team level, it can also help separate concerns. You can treat it as an in-process microservice.