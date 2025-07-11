# Optional

The Rust type `Option<T>` is also an enum, defined like this:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Optional replaces `null` in Rust. It is used when a value may or may not be present. This avoids having a sentinel value (that you might accidentally use) or a null pointer dereference.

```rust
fn main() {
    let x: Option<i32> = None;
    let y: Option<i32> = Some(42);

    // You can `match` on optionals - just like any other enum.
    match x {
        None => println!("x is None"),
        Some(v) => println!("x is {}", v), // Notice we "capture" v from the enum
    }

    // You can use `if let` to only match one case
    if let Some(v) = y {
        println!("y is {}", v);
    }

    // You can use `unwrap` to get the value out, but it will panic if it's None
    println!("y is {}", y.unwrap());

    // You can use `unwrap_or` to provide a default value
    println!("x is {}", x.unwrap_or(0));

    // You can use `unwrap_or_else` to provide a default value via a closure
    println!("x is {}", x.unwrap_or_else(|| 1 + 1));

    // You can use `map` to transform the value inside the option (useful for chains of operations)
    let z = y.map(|v| v + 1);
    println!("z is {:?}", z);

    // You can also use `let else` syntax to handle the None case early
    let Some(v) = y else {
        println!("y is None");
        return;
    };
    println!("y is {}", v);
}
```

