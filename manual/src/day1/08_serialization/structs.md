# Serializing and Deserializing Structs

## Setup Dependencies

Add the following to your *workshop dependencies* in `code/Cargo.toml`:

```toml
[workspace.dependencies]
serde = { version = "1.0.219", features = ["derive"] }
```

That's the first time we've mentioned "features". *Feature flags* are a way to enable optional functionality in a crate. In this case, the `derive` feature enables the ability to automatically generate code for serializing and deserializing structs using Rust's `derive` attribute.

Let's also add support for JSON (Serde doesn't support any formats out of the box):

```toml
serde_json = "1.0.140"
```

Now we'll make a project to play with serialization:

```bash
cargo new serialize
```

And we'll add the dependencies to `code/day1/serialize/Cargo.toml`:

```toml
[dependencies]
serde.workspace = true
serde_json.workspace = true
```

## Serlialize and Deserialize

> See `code/day1/serialize` for the code.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    a: u32,
    b: String,
    c: Option<f64>,
    d: Vec<u8>,
    e: Box<i32>,
    missing: Option<String>,
}

fn main() {
    // Make an example set of data
    let starting_data = MyData {
        a: 42,
        b: "Hello, world!".to_string(),
        c: Some(3.14),
        d: vec![1, 2, 3, 4, 5],
        e: Box::new(7),
        missing: None,
    };
    println!("Original struct:\n{:#?}", starting_data);

    // Serialize it to a JSON string
    let json_string = serde_json::to_string_pretty(&starting_data).unwrap();
    println!("Serialized JSON:\n{}", json_string);

    // Deserialize it back to a Rust struct
    let deserialized_data: MyData = serde_json::from_str(&json_string).unwrap();
    println!("\nDeserialized struct:\n{:#?}", deserialized_data);
}
```

It really is that straightforward. The `derive` attribute generates the code to serialize and deserialize the struct; Serde has support for most of the standard library types, and can be extended to support your own types.

> Tip: MANY crates implement a `serde` feature flag.