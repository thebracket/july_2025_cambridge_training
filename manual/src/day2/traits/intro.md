# Traits

You've used a lot of traits already, but we haven't talked about them in any detail.

When you type:

```rust
#[derive(Debug)]
```

You are using the `Debug` trait, which is a built-in trait that allows you to format a type for debugging output.

Traits are basically an *interface* in other languages, or a contract. They can't hold data, but they can define methods that types can implement. You can then use these methods on any type that implements the trait.

For example:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

Actually expands to:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}
```
