# Question Mark Operator

The `?` operator is a *lot* like `throw` - but it's just syntax sugar for `return Err(...)` in a function that returns a `Result`.

## Polymorphic Errors!

We'll start with something simple:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn main() {
    match maybe_read_a_file() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

Easy enough - so let's add a function that reads a file and converts to to uppercase:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

fn main() {
    match file_to_uppercase() {
        Ok(text) => println!("File contents: {text}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

So far, so good. So let's add a function that reads the file and then deserializes it:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn deserialize() -> Result<serde_json::Value, serde_json::Error> 
{
    let data = maybe_read_a_file()?;
    let json: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json)
}

fn main() {
    match deserialize() {
        Ok(json) => println!("Deserialized JSON: {json}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

This won't compile. We've got a problem: `maybe_read_a_file` returns a `Result<String, std::io::Error>`, but `deserialize` is trying to return a `Result<serde_json::Value, serde_json::Error>`.

The answer is to use a `Box<dyn std::error::Error>` as the error type:

```rust
use std::path::Path;

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn deserialize() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let data = maybe_read_a_file()?;
    let json: serde_json::Value = serde_json::from_str(&data)?;
    Ok(json)
}

fn main() {
    match deserialize() {
        Ok(json) => println!("Deserialized JSON: {json}"),
        Err(e) => println!("An error occurred: {e:?}"),
    }
}
```

> You'd be amazed how many times Rust users type: `Result<T, Box<dyn std::error::Error>>` as the error type!