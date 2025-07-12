# Awaiting

What does this do?

```rust
async fn my_async_function() -> u32 {
    println!("Hello from my_async_function!");
    42
}

#[tokio::main]
async fn main() {
    let result = my_async_function();
}
```

The answer: nothing at all! The `async` in the function signature transforms the function so that it returns a `Future` - a unit of future work. It won't be submitted to the runtime until you `.await` it in some way!

> There's a lint that will warn you if you call an async function and don't `.await` it. This is a common mistake!

To actually run the async function, you need to `.await` it:

```rust
async fn my_async_function() -> u32 {
    println!("Hello from my_async_function!");
    42
}

#[tokio::main]
async fn main() {
    let result = my_async_function().await;
    println!("Result was: {}", result);
}
```

So what actually happens here?
1. `my_async_function` is called, which returns a `Future`.
2. The `.await` on the future:
    - Registers the future with the runtime, so it can be polled.
    - Yields control back to the runtime, allowing other tasks to run while waiting for this one to complete.
3. When the future is ready (in this case, immediately), it resumes execution, printing "Hello from my_async_function!" and returning `42`.
4. The runtime then resumes the `main` function, printing "Result was: 42".

That's a lot of yielding and resuming - but it's all very fast. (Go famously automatically awaits on every function call!)

## Joining Futures

Sometimes you want to run multiple async tasks concurrently, and wait for all of them to complete. You can do this with `tokio::join!`:

```rust
use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 complete");
}

async fn task2() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 complete");
}

#[tokio::main]
async fn main() {
    tokio::join!(task1(), task2());
    println!("Both tasks complete");
}
```

> This code is in `code/day3/async_join` if you want to try it out.

## Selecting Futures

Sometimes you want to wait for the first of multiple futures to complete. You can do this with `tokio::select!`:

```rust
use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 complete");
}

async fn task2() {
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 complete");
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = task1() => println!("Task 1 finished first"),
        _ = task2() => println!("Task 2 finished first"),
    }
    println!("One task complete");
}
```

This is most useful when you have multiple sources of input, and want to respond to whichever one is ready first. One source of input can be a timer, another a network socket, a third a channel, etc.

## Spawning Tasks

You can spawn tasks to run concurrently using `tokio::spawn`:

```rust
use tokio::time::{sleep, Duration};
async fn my_task() {
    sleep(Duration::from_secs(2)).await;
    println!("Task complete");
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(my_task());
    println!("Task spawned, doing other work...");
    handle.await.unwrap(); // Wait for the task to complete
    println!("Main done");
}
```