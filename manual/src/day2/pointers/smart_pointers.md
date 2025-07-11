# Quick Smart Pointer Overview

Most of the time, when you're working with pointers in Rust you won't ever see actual pointer syntax. You'll use smart pointers:

|Type|C++ Analogue|Description|
|---|---|---|
|`Box<T>`|`std::unique_ptr<T>`|A heap-allocated value. It is the most common smart pointer.|
|`Rc<T>`|`std::shared_ptr<T>`|A reference-counted value. It allows multiple ownership of the same value.|
|`Arc<T>`|`std::shared_ptr<T>`|An atomic reference-counted value. It is thread-safe and allows multiple ownership across threads.|

> If you really like this stuff, check out [Learn Rust with Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html).

`Rc` is single-threaded, `Arc` is multi-threaded. It's relatively unusual to need `Rc` in modern systems!

# Example of Box

Here's a simple example of using `Box`:

```rust
fn main() {
    let x = Box::new(5); // Allocate an integer on the heap
    println!("x = {}", x);
}
```

That's it! You put a value on the heap, and you can use it like a normal value. The memory is automatically freed when `x` goes out of scope.

# Example of Arc

Here's a simple example of using `Arc`:

```rust
use std::sync::Arc;

struct SharedData {
    value: i32,
}
impl Drop for SharedData {
    fn drop(&mut self) {
        println!("Dropping SharedData with value: {}", self.value);
    }
}

fn do_something(data: Arc<SharedData>) {
    println!("Doing something with value: {}", data.value);
}

fn main() {
    let data = Arc::new(SharedData { value: 42 });
    let data_clone = Arc::clone(&data); // Clone the Arc to share ownership

    do_something(data_clone);
    println!("SharedData is still alive after do_something");
}
```

Notice that when we run this: the data is dropped *once*. We called `clone` several times. On an `Arc`, clone just increments the reference count. When the last `Arc` goes out of scope, the data is dropped. Cloning an `Arc` is cheap, as it only increments a counter. (`Rc` is even cheaper, as it doesn't need to be thread-safe.)