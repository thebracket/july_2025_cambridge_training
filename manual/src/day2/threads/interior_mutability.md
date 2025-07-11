# Interior Mutability

We talked about `Sync+Send`, and it's great to be able to serialize access to data---and become immune to data races. Sometimes, you want to mutate *parts* of data---possibly independently.

`Sync` is an automatic property. If everything in a `struct` is `Sync`, then the `struct` is `Sync`.

> You can actually implement `Sync` yourself in an `unsafe` block and pinky swear that you won't break it. But you really shouldn't do this unless you know what you're doing!

Take the following example:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

struct SharedDoc {
    counter: AtomicUsize,
    shared_string: Mutex<String>,
    shared_int: Mutex<i32>,
}

fn is_it_sync<T: Sync>() {
    println!("Yes, this type is Sync!");
}

fn main() {
    let doc = SharedDoc {
        counter: AtomicUsize::new(0),
        shared_string: Mutex::new(String::new()),
        shared_int: Mutex::new(0),
    };

    // Increment the counter atomically
    doc.counter.fetch_add(1, Ordering::SeqCst);

    // Lock the mutex to mutate the shared string
    {
        let mut string = doc.shared_string.lock().unwrap();
        string.push_str("Hello, world!");
    }

    // Lock the mutex to mutate the shared integer
    {
        let mut int = doc.shared_int.lock().unwrap();
        *int += 1;
    }

    is_it_sync::<SharedDoc>();
}
```

This compiles and works fine. You now have a structure with fine-grained locking. It's `Sync`!


```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

struct SharedDoc {
    counter: AtomicUsize,
    shared_string: Mutex<String>,
    shared_int: Mutex<i32>,
}

fn main() {
    let doc = SharedDoc {
        counter: AtomicUsize::new(0),
        shared_string: Mutex::new(String::new()),
        shared_int: Mutex::new(0),
    };

    for _ in 0..10 {
        thread::scope(|s| {
            s.spawn(|| {
                // Increment the counter atomically
                doc.counter.fetch_add(1, Ordering::SeqCst);
            });
        });
    }

    println!("Counter: {}", doc.counter.load(Ordering::SeqCst));
}
```

It's `Send` too - you can use it across threads. Because each thread closure *captures* the `doc` variable *immutably* --- it's relying on the interior mutability of the `Mutex` and `AtomicUsize` to mutate the data safely --- you can access it across multiple threads without risk of data races.