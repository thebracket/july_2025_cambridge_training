# Slices

Slices are a view into a contiguous sequence of elements in a collection. They are much like (element *), size in C, or a view in C++. They work with any collection that is stored in a contiguous block of memory, such as arrays and vectors.

Slices are a reference type, and they are always borrowed. This means that you cannot create a slice without borrowing the collection it is a view into.

```rust
fn print_slice(slice: &[i32]) {
    for i in slice {
        println!("{}", i);
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    print_slice(&v); // Print the whole vector
    print_slice(&v[1..4]); // Print a slice of the vector
}
```

Slices can be mutable, but only if the collection they are a view into is mutable. This means that you cannot create a mutable slice from an immutable collection.

```rust
fn add_one(slice: &mut [i32]) {
    for i in slice {
        *i += 1;
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    add_one(&mut v);
    print_slice(&v);
}
```
