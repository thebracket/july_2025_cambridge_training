# Other Channel Types

The standard library includes `mpsc` channels. There are some very commonly used alternatives, including:

* `crossbeam::channel` - a drop-in replacement for `std::sync::mpsc` that is faster, supports multiple producers and multiple consumers, and has more features like `select!` to wait on multiple channels at once. It is widely used in the Rust ecosystem. You can clone the `rx` and the `tx` for multiple consumers and producers.
* `flume` - another popular channel library with a focus on ergonomics and performance. It supports bounded and unbounded channels, multiple producers and consumers, and has a similar API to `mpsc`.
* `oneshot` - a simple channel for sending a single value from one thread to another. It is useful for cases where you only need to send one message and want a very lightweight solution.
