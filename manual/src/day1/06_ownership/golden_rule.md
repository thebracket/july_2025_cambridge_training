# The Golden Rule: One Mutable or Many Immutable

A hard-and-fast rule of Rust is:

> You can have either one mutable reference (exclusive) or any number of immutable references to a value.

This is the golden-rule of the borrow checker. It prevents data-races (we'll look at those soon), and it prevents whole classes of bugs that are common in other languages.

## So Why Does THIS work?

```rust
fn main() {
    let mut s = String::from("Hello");
    let a = &s; // Immutable borrow
    let b = &s; // Immutable borrow
    println!("{} and {}", a, b);
    let c = &mut s; // Mutable borrow
    c.push_str(", world!");
    println!("{}", c);
}
```

I literally just told you that you can't have mutable and immutable references at the same time - but this compiles and runs just fine! Rust tracks the scope of the references. Since `a` and `b` are no longer used after the first `println!`, the Rust compiler can prove that they are no longer in use, and allows the mutable borrow to occur.

The Rust compiler has been improving rapidly on this front. This wouldn't have compiled a few years ago.

However, this code will not compile:

```rust
fn main() {
    let mut s = String::from("Hello");
    let a = &s; // Immutable borrow
    let b = &s; // Immutable borrow
    let c = &mut s; // Mutable borrow
    println!("{} and {}", a, b);
    c.push_str(", world!");
    println!("{}", c);
}
```

`a` and `b` are still in scope when we try to create the mutable borrow `c`. This is a compile-time error.

## Takeaway: The borrow checker is your friend

The borrow checker is helping prevent nasty bugs and data corruption. It can be frustrating at first, but it gets easier. Try to limit your use of mutable references, and prefer immutable references. Keep your scopes small, and favor working step-by-step over trying to do everything at once. (This is good advice for programming in general!)

## And Can Still Be Frustrating

In-place moving with iterators tends to cause some confusion.