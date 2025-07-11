# Mutability

Coming from other languages, you'd probably expect this to work:

```rust
fn main() {
    let x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is now {x}");
}
```

This code won't compile! Rust variables are *immutable by default*. This works fine:

```rust
fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is now {x}");
}
```

> *Advanced note!* There's been a long debate in the Rust community about whether `mut` actually stands for "mutable" or "compile time mutex". We'll discuss this when we get into ownership and concurrency.