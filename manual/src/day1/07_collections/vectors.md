# Vectors

Vectors are the most common collection in Rust (I frequently hear new users from C grumble that "it's vectors everywhere"). Vectors are very similar to `vec` in C++, or `List` in Python.

In-memory, a vector contains a pointer to a contiguous block of memory, a length, and a capacity. The length is the number of elements in the vector, and the capacity is the amount of memory that has been allocated for the vector. The capacity is always greater than or equal to the length.

Just like C++ and Python, when a vector runs out of capacity, it will allocate a new block of memory, copy the elements to the new block, and free the old block. The capacity strategy is implementation-defined, but it is typically doubled each time the vector runs out of capacity.

Using vectors is easy:

```rust
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);
    println!("Length: {}", v.len());
    println!("Capacity: {}", v.capacity());

    // You can also use the vec! macro to create a vector with initial values
    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    println!("Vector 2: {:?}", v2); 

    // You can reserve capacity if you know how many elements you'll need
    let mut v3 = Vec::with_capacity(10);
    println!("Vector 3: {:?}, Capacity: {}", v3, v3.capacity());

    // You can shrink the capacity to fit the length
    v3.push(1);
    v3.push(2);
    v3.shrink_to_fit();
    println!("Vector 3: {:?}, Capacity: {}", v3, v3.capacity());

    // You can also pop elements off the end of the vector
    let last = v.pop();
    println!("Popped: {:?}, Vector: {:?}", last, v);

    // You can access elements by index (this will panic if the index is out of bounds)
    // It will NOT segfault, like C/C++.
    println!("First element: {}", v[0]);

    // You can also use get() to safely access elements
    match v.get(10) {
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("No element at index 10"),
    }

    // You can iterate over the elements in a vector
    for i in &v {
        println!("Element: {}", i);
    }

    // You can also iterate over the elements in a vector with index
    for (i, value) in v.iter().enumerate() {
        println!("Element {}: {}", i, value);
    }

    // You can clear a vector, which will drop all the elements
    v.clear();
    println!("Vector after clear: {:?}", v);
    println!("Length: {}, Capacity: {}", v.len(), v.capacity());

    // You can extend a vector with another vector
    v.extend(&v2);
    println!("Vector after extend: {:?}", v);

    // Use "retain" to remove elements that don't match a predicate
    v.retain(|&x| x % 2 == 0);
    println!("Vector after retain: {:?}", v);

    // You can also remove elements by index
    v2.remove(0);
    println!("Vector 2 after remove: {:?}", v2);

    // You can insert elements at a specific index (it's slow, because it has to move elements around)
    v2.insert(0, 10);
    println!("Vector 2 after insert: {:?}", v2);
}
```
