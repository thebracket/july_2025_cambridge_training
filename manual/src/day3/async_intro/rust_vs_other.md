# Rust Async vs Other Languages

Rust's async model is based on the concept of "futures" and "async/await" syntax, similar to many modern languages. However, Rust and C++ are systems programming languages that need to scale from embedded to supercomputers. Both Rust and C++ chose to be "runtime agnostic", and just include enough language support to let you write async(co-routine) code.

Most other languages have an opinionated runtime built-in:

|Language|Runtime Model|
|---|---|
|JavaScript (nodejs)|Single-threaded event loop, with async I/O|
|Python (asyncio)|Single-threaded event loop, with async I/O|
|C#/.NET|Thread pool with async/await, async I/O|
|Go|Goroutines, multiplexed on OS threads, async I/O|
|Java|Thread pool, async I/O (CompletableFuture)|
|Ruby (async gems)|Single-threaded event loop, async I/O|

Rust and C++ both require you to pick a runtime library (like Tokio, async-std, smol, or others for Rust; or Boost.Asio, libuv, etc. for C++). This gives you more flexibility, but also means you have to make more decisions.

With `embassy`, Rust can even do async on embedded without a full OS, which is not possible in most other languages!