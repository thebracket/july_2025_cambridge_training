# Panic Handling

Your program doesn't have to self-destruct on panic. You can catch panics and recover from them, allowing your program to continue running or to shut down gracefully.

## Catching Panics

You can catch panics using `std::panic::catch_unwind`. This function takes a closure and returns a `Result`, which will be `Ok` if the closure executed without panicking, or `Err` if a panic occurred.

```rust
use std::panic;



fn main() {
    panic::set_hook(Box::new(|panic_info| {
        println!("Custom panic hook");
        println!("Panic message: {:?}", panic_info);
    }));

    let result = panic::catch_unwind(|| {
        panic!("This is a panic!");
    });

    match result {
        Ok(_) => println!("No panic occurred."),
        Err(_) => println!("Caught a panic!"),
    }
}
```

> Every now and again, someone proposes adding `try` and `catch` into Rust. It's actually not as hard as it sounds, because the LLVM system uses `libexception` under the hood for the unwinding!