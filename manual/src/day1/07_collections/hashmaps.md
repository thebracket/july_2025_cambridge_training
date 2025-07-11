# HashMaps

While vectors are your traditional array-like collection, HashMaps are a key/value store. They are similar to dictionaries in Python, maps in Go, or objects in JavaScript, or maps in C++.

HashMaps:

* Are not ordered.
* Are not indexed by number.
* Are not stored on the stack (the HashMap itself is, but the data is on the heap).
* Are resizable.
* Are homogeneous (all keys are the same type, all values are the same type).

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    println!("{:?}", map);

    // Accessing values
    if let Some(value) = map.get("key1") {
        println!("key1: {}", value);
    }

    // Iterating over key/value pairs
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Removing a key/value pair
    let previous_value = map.remove("key2"); // returns Option<V>
    println!("Removed key2, previous value: {:?}", previous_value);

    // Check if a key exists
    if map.contains_key("key3") {
        println!("key3 exists");
    }

    // Clearing the map
    map.clear();

    // HashMaps can be created from a vector of tuples
    let mut map2: HashMap<&str, &str> = vec![("a", "1"), ("b", "2"), ("c", "3")]
        .into_iter()
        .collect();
    println!("{:?}", map2);

    // You can also use the entry API to insert or update values
    map2.entry("d").or_insert("4");
    map2.entry("a").and_modify(|v| *v = "5");
    println!("{:?}", map2);
}
```

There's the related `HashSet` type which has just keys.
