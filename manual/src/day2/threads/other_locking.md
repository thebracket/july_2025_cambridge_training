# Other Locking Primitives

The other commonly used locking primitive is `RwLock`. `RwLock` allows multiple readers or one writer at a time, making it more flexible than a `Mutex` in scenarios where reads are more common than writes.

It's also *much* easier to deadlock with `RwLock`, so use it carefully! `RwLock` is great for rarely updated caches, because readers aren't serialized. But a `write()` will block until all readers leave---and then block all readers until the write is done. That makes it *very* easy to deadlock if you're not careful.