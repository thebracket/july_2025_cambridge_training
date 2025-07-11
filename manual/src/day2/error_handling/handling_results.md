# Handling Results

You've already seen some error handling in Rust. Let's add a little more detail:

```rust
fn main() {
    let result: Result<i32, &str> = Ok(42);

    // You can match - it IS an enum
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Helper functions like `is_ok` and `is_err`
    if result.is_ok() {
        println!("Result is OK");
    } else {
        println!("Result is an error");
    }

    // If the error is fatal, you can panic the program
    result.unwrap(); // This will panic if it's an Err
    // Or you can use `expect` to provide a custom panic message
    result.expect("Expected a successful result, but got an error");
    
    // You can also use `map` to transform the Ok value
    let transformed = result.map(|v| v * 2);
    match transformed {
        Ok(value) => println!("Transformed value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // You can use `map_err` to transform the error value into a DIFFERENT error type
    let result_with_different_error: Result<i32, String> = result.map_err(|e| e.to_string());
    match result_with_different_error {
        Ok(value) => println!("Value with different error type: {}", value),
        Err(e) => println!("Error with different type: {}", e),
    }

    // You can use `and_then` to chain operations that return a Result
    let chained_result = result.and_then(|v| {
        if v > 0 {
            Ok(v * 2)
        } else {
            Err("Value must be positive")
        }
    });
}
```

> These are mostly provided for reference.