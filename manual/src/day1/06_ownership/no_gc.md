# No Garbage Collector

Rust does not have a garbage collector. In Python or Java, you create an object - and it will be cleaned up automatically (at some point) by the garbage collector. This is convenient, but it has a few problems:

* Garbage collection can cause unpredictable pauses in your program. This is especially bad for games, real-time systems, and high-performance applications.
* Garbage collection can cause memory bloat. If your program creates a lot of objects, the garbage collector may not run often enough to free up memory.
* Garbage collection can make it hard to reason about the performance of your program. You never know when the garbage collector will run, or how long it will take.

Rust uses a different approach: ownership. Every value in Rust has a single owner, and when the owner goes out of scope, the value is "dropped". This is called RAII (Resource Acquisition Is Initialization), and it's a common pattern in C++.

## We can demo this with a simple program:

```rust
struct MyStruct {
    data: String,
}
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

fn main() {
    let my_struct = MyStruct {
        data: String::from("Hello, world!"),
    };
    // my_struct goes out of scope here
}
```

When you run this program, you'll see the message "Dropping MyStruct with data: Hello, world!" printed to the console when `my_struct` goes out of scope. This is because Rust automatically calls the `drop` method when the value is no longer needed.

If you wanted to put `MyStruct` on the heap, you could use `Box`:

```rust
struct MyStruct {
    data: String,
}
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

fn main() {
    let my_struct = Box::new(MyStruct {
        data: String::from("Hello, world!"),
    });
    // my_struct goes out of scope here
}
```

Dropping is transitive. If you have a struct that contains other structs, Rust will drop the inner structs first, then the outer struct. This is important to understand when working with complex data structures.

> Rust does not *guaranty* that you won't have memory leaks. That's outside of its safety guarantees (and there are even some commands to deliberatly leak memory!). It's very rare to achieve a memory leak in Rust.