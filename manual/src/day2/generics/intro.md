# Generics

Generics are a powerful feature in Rust that allow you to write flexible and reusable code. The best part - you've already made a generic function!

When you created:

```rust
fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}
```

It's syntax sugar for a generic version of the function. The `impl Animal` part means that the function can accept any type that implements the `Animal` trait, making it generic over the type of `animal`.

This is the same thing:

```rust
fn speak_twice<T: Animal>(animal: &T) {
    animal.speak();
    animal.speak();
}
```