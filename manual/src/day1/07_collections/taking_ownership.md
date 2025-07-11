# Taking Ownership

Using `.iter()` on a vector borrows the vector, and gives you references to the elements.

You can also use `.into_iter()` to take ownership of the elements. This is useful if you want to move the elements out of the vector, or if you want to consume the vector.

```rust
fn main() {
    let v = vec![String::from("Hello"), String::from("World")];
    for s in v.into_iter() {
        println!("{}", s);
    }
    // println!("{:?}", v); // This won't compile, because v has been moved
}
```
