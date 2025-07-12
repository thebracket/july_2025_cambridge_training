# Async Critique

Async Rust has some strong critics. [Async Rust Is A Bad Language](https://bitbashing.io/async-rust.html) is a well-known article that outlines many of the common complaints. Many of these complaints are valid---and async should be used judiciously.

> Imagine my fear when I discovered I was training a class in the same building as the author of that article!

## Common Complaints

### Function Coloring

A normal function cannot call an async function without spawning a runtime. You cannot await in a non-async function! This means that async functions tend to "infect" the call graph, and you end up with a lot of `async fn` sprinkled around. You also find yourself with library code that offers both async and sync versions of functions. Messy!

The standard approach to avoiding this is to try and write "protocol" code in an agnostic fashion, requiring the calling code to write either async or sync wrappers. We took that approach in the MUD workshop - the protocol system just makes bytes, it's up to the caller to decide how to read and write them.

### Sync+Send

Tokio in particular is notorious for making you put `Sync + Send` bounds on everything. Passing references between async tasks which can (and will) move, is not fun - and typically won't compile (you can use `pin`).

You don't have this problem in garbage collected languages, because the garbage collector ensures that references are always valid. Rust's ownership model makes this more difficult, but also safer.

The common solution is to use `Arc` everywhere. This isn't as bad as it sounds - `Arc` is cheap, and you're effectively implementing a garbage collector with deterministic cleanup (it won't pause the world).

### It's Threads Under The Hood, Anwyay

This *is* true. Async runtimes like Tokio use threads under the hood. Even in `current_thread` mode, blocking tasks will use a thread from Tokio's thread pool for I/O operations that don't support async natively.

## Takeaway

My takewaways are:

* Async is *great* for I/O bound workloads, especially when you have many connections. It can be more efficient than threads, and it scales better.
* Async is *not* great for CPU bound workloads. If you have a lot of CPU work to do, threads are probably better.
* Async is *not* great for beginners, making the learning curve steeper.

And most importantly:

You can have both! Pick what makes sense for your workload.