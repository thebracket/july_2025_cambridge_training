# Operators

> "Operator overloading" got a bad name from C++. You *can* abuse it, and decide that operators do bizarre things. Please don't. If you allow two types to be added together, please use an operation that makes sense to the code reader!

You can implement operators for your types. Let's make a `Point` type that can be added together:

```rust
use std::ops::Add;

struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x, 
            y: self.y + rhs.y
        }
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;
    println!("c.x = {}, c.y = {}", c.x, c.y);
}
```

There's a full range of operators you can overload. You can also overload the `+=`, `/`, `*` operators, and so on. This is very powerful for letting you express functions (rather than remembering to add `x` and `y` each time)---but it can be abused horribly if you decide that `+` should mean "subtract" or something. Don't do that. Please.