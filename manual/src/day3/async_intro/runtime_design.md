# Runtime Design Overview

`Tokio` (tokenized i/o) has emerged as the dominant async runtime for Rust. It's very full-featured, regularly updated and maintained, and has a large ecosystem of libraries built on top of it. It supports both single-threaded and multi-threaded runtimes, and has a lot of features for working with timers, networking, file I/O, and more.

There *are* faster, more focused runtimes like `smol`, and runtimes that focus on `io_uring` on Linux. Most of the concepts here will apply to other runtimes, but the examples will be Tokio specific.

## Initializing Tokio

The simplest way to use Tokio is to use the `#[tokio::main]` attribute on your main function. This will set up a multi-threaded runtime for you automatically:

```rust
#[tokio::main]
async fn main() {
    // Your async code here
}
```

You can also create a single-threaded runtime with `#[tokio::main(flavor = "current_thread")]`:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Your async code here
}
```

If you need more control, you can create a runtime manually:

```rust
use tokio::runtime::Runtime;

let rt = Runtime::new().unwrap();
rt.block_on(async {
    // Your async code here
});
```

There is *nothing* to stop you from spawning a thread and putting a Tokio runtime in it. This is actually quite a common pattern when you have a naturally synchronous application that needs to do some async work.

You can even have multiple runtimes in the same application, though you have to be careful about which runtime is used for which tasks. Spawning tasks on the wrong runtime can lead to confusing bugs!