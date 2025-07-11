# Arrays

Arrays are the simplest collection type. They are:

* Fixed size.
* Homogeneous (all elements are the same type).
* Stored on the stack.
* Accessed by index.

## Array Example

```rust
fn main() {
    let declared_array = [1, 2, 3];
    let typed_array: [i32; 3] = [1, 2, 3];
    let initialized_array = [0; 10]; // ten zeroes

    println!("declared_array: {:?}", declared_array);
    println!("typed_array: {:?}", typed_array);
    println!("initialized_array: {:?}", initialized_array);

    // Accessing elements
    println!("First element: {}", declared_array[0]);
    for i in 0..declared_array.len() {
        println!("Element {}: {}", i, declared_array[i]);
    }

    // Accessing elements by iteration
    for element in &declared_array {
        println!("Element: {}", element);
    }

    // Accessing elements by iteration with index
    for (index, element) in declared_array.iter().enumerate() {
        println!("Element {}: {}", index, element);
        // You can also modify the elements if needed (make the array mutable!)
    }

    // Accessing out of bounds will panic at runtime
    // println!("Out of bounds: {}", declared_array[3]); // Uncommenting this line to panic

    // Instead, use get() to safely access elements
    match declared_array.get(3) {
        Some(value) => println!("Element at index 3: {}", value),
        None => println!("No element at index 3"),
    }
}
```

# Limitations

* Arrays are fixed size, and the size must be known at compile time.
* Arrays are not resizable. If you need a resizable collection, use a `Vec`.
* Arrays do not implement many of the useful traits that other collections do, such as `IntoIterator` (without a reference), `Extend`, or `FromIterator`.
* Arrays live on the stack, so they are limited in size by the stack size. If you need a large collection, use a `Vec` or another heap-allocated collection (or `SmallVec` from crates.io).