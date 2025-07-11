# Let's Try That in Rust

An equivalent Rust program would look like this:

```rust
use std::thread;

fn main() {
    let mut counter = 0;

    // Scoped threads automatically join when they go out of scope,
    // and avoid exit-ordering problems.
    thread::scope(|s| {
        for _ in 0..3 {
            s.spawn(|| {
                for _ in 0..100_000 {
                    counter += 1; // Unsafe because we're accessing a static mutable variable
                }
            });
        }
    });
}
```

This won't compile. You cannot borrow `counter` mutably more than once at a time - so the program won't compile.

You *can* actually force Rust to emulate the same bug with some `unsafe` code (please don't do this in real code!):

```rust
// As of Rust 2024, we have to issue a compiler directive to allow static mutable references at all!
#![allow(static_mut_refs)]
use std::thread;

static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = vec![];

    for _ in 0..3 {
        handles.push(thread::spawn(|| {
            unsafe {
                for _ in 0..100_000 {
                    COUNTER += 1; // Unsafe because we're accessing a static mutable variable
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        println!("Counter: {}", COUNTER);
    }
}
```

You have to use `unsafe` to access the static mutable variable `COUNTER`. That's there because in some embedded systems, you can't avoid static mutables.