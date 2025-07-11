# The ToString Example

Let's make a really handy generic function:

```rust
fn print_string<T: ToString>(value: T) {
    let v = value.to_string();
    println!("{}", v);
}

fn main() {
    print_string("Hello, world!");
    print_string(42);
    let s = String::from("Hello, Rust!");
    print_string(s);
}
```

This function takes any type that implements the `ToString` trait, converts it to a string, and prints it. It's a handy pattern when you'd like your users to stop having to type `to_string()` all the time!

More importantly, it illustrates how generics work in Rust. The `T: ToString` syntax means that `T` can be any type that implements the `ToString` trait. This allows us to write code that works with multiple types without duplicating code.

> Internally, your code *is* duplicated. The compiler will make a separate version of the function for each type you use it with. This is called "monomorphization". It's a powerful optimization that Rust does automatically, so you don't have to worry about it. If you *really* overdo the number of types, you might end up with a large binary, but in practice, this is rarely an issue.