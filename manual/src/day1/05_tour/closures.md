# Closures

Closures are anonymous functions that can capture their environment. They are often used for short-lived operations, such as passing a function as an argument to another function.

```rust
fn main() {
    let x = 5;
    let print_x = || println!("x is {x}");
    print_x();
}
```

We used them in the primitive types section in the context of an `or_else`. This is a common pattern:

```rust
fn main() {
    let x: Option<i32> = None;
    let y = x.unwrap_or_else(|| 42);
    println!("y is {y}");
}
```

It's really common to use closures with iterators:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
}
```

> This is one area in which I wish Rust were more C++-like. In C++, you specify the capture rules explicitly - Rust will happily just grab whatever it needs. Be careful with this!
