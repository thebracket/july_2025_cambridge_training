# Mutexes

Rust has Mutexes, too. Unlike many other languages, Rust's Mutexes *wrap* the data they protect --- making it (almost) impossible to forget to lock the mutex before accessing the data.

```rust
fn main() {
    let counter = std::sync::Mutex::new(0);

    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
        }
    });

    println!("Counter: {}", counter.lock().unwrap());
}
```

There's a few things to note here!

* The `Mutex` is generic, wrapping *around* the data it protects.
* You have to call `lock()` to access the data. This returns a `MutexGuard`, which is a smart pointer that automatically unlocks the mutex when it goes out of scope.
* The `lock()` method can fail, so you should handle the error. In this example, we use `unwrap()` to panic if it fails, but in production code you should handle the error gracefully. (If it fails, a *poisoning* error has occurred, meaning another thread panicked while holding the lock. This is a way to prevent data corruption.). A lot of places use the `parking_lot` crate because it has a more ergonomic API and better performance.
* You can try really hard, but without getting into memory escapades - you're not getting to the protected data without locking the mutex first. This is a big win for safety!

There's also one *big* thing note here.

```rust
let counter = std::sync::Mutex::new(0);
```

Notice that there's no `mut`. As I mentioned yesterday, `mut` doesn't *really* mean "this is mutable". It means "this is mutable *in this scope*" - and is effectively a compile-time Read-Write Lock to enforce the borrowing rules.

What's *really* going on? Rust has two "automatic traits", `Sync` and `Send`.

`Send` means that a type can be safely *read* across threads. Almost everything is `Send`, except for types that contain non-thread-safe data (like `Rc`).

`Sync` means that a type can "take over" from the borrow checker with a runtime check. `Mutex` enforces that you can only have one accessor at a time, so it implements `Sync`. This means that you can have a `Mutex<T>` in a thread, and it can be shared across threads. The `Mutex` will ensure that only one thread can access the data at a time. (There are some scary types like `RefCell` that let you do this by hand. Be careful!)

## Deadlocks

Mutexes can lead to deadlocks if not used carefully. A deadlock occurs when two or more threads are waiting for each other to release a lock, causing them to be stuck indefinitely.

Here's a deadlock (please don't run this):

```rust
use std::sync::Mutex;

fn main() {
    let lock1 = Mutex::new(0);
    let lock = lock1.lock().unwrap();
    let lock = lock1.lock().unwrap(); // This will deadlock!
}
```

> Rust DOES NOT help you here! There are some projects in the works to provide some safety, but they are not yet stable. You can use the `deadlock` crate to detect deadlocks at runtime, but it is not a substitute for careful design.

Some rules to avoid deadlocks:
1. **Lock order**: Always acquire locks in a consistent order. If multiple threads need to lock multiple mutexes, ensure they always lock them in the same order.
2. **Try_lock**: Use `try_lock()` instead of `lock()` when possible. This allows you to attempt to acquire a lock without blocking, and handle the case where the lock is already held.
3. **Don't Return MutexGuards, Ever**: Avoid returning `MutexGuard`s from functions. This can lead to situations where a lock is held longer than intended, increasing the risk of deadlocks.
4. **Hide Your Mutexes**: If you can, hide your mutexes behind a type that only exposes the methods you need. This way, you can control how the mutex is accessed and reduce the risk of deadlocks.
