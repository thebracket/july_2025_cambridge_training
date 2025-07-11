# Result

`Result` is very similar to `Option`, but is used for functions that can fail. It is defined like this:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The convention is that `Ok` is used for successful results, and `Err` is used for errors. The `E` type is typically an enumeration of possible errors, a boxed error type (we'll worry about that soon!), or a string.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // You can use `unwrap` to get the value out, but it will panic if it's an error
    println!("Result: {}", divide(4.0, 2.0).unwrap());

    // You can use `unwrap_or` to provide a default value
    println!("Result: {}", divide(4.0, 0.0).unwrap_or(0.0));

    // And all the other options we had for Option!
}
```

We're going to do a deeper dive into error handling later.