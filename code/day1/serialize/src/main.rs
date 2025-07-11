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
