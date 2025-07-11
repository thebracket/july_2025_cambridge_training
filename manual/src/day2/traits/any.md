## The `Any` Type

If you really, *really* need to find out the concrete type of a dynamically dispatched trait, you can use the `std::any::Any` trait. It's not the most efficient design, but it's there if you *really* need it.

The easiest way to "downcast" is to require `Any` in your type and an `as_any` function:

```rust
struct Tortoise;

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

Then you can "downcast" to the concrete type:

```rust
let more_animals : Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
for animal in more_animals.iter() {
    if let Some(cat) = animal.as_any().downcast_ref::<Tortoise>() {
        println!("We have access to the tortoise");
    }
    animal.speak();
}
```

If you can avoid this pattern, you should. It's not very Rusty---it's pretending to be an object-oriented language. But it's there if you need it.