# Define A Trait

Let's define a simple trait:

```rust
trait Animal {
    fn speak(&self);
}
```

Notice that we've named the trait, and we're requiring that any type that implements this trait must provide a `speak` method.

So now we can implement this trait for a specific type, like `Dog`:

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let dog = Dog;
    dog.speak(); // Outputs: Woof!
}
```

## Traits as Function Parameters

You can also create functions that require that a parameter implement a trait:

```rust
fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}
```

You can call it with `speak_twice(&cat)`---and it runs the trait's function twice.

## Traits as Return Types

You can also return a trait from a function:

```rust
fn get_animal() -> impl Animal {
    Cat
}
```

The fun part here is that you no-longer know the concrete type of the returned type---you know for sure that it implements `Animal`. So you can call `speak` on it, but if `Cat` implements other traits or functions, you can't call those functions.

## Traits that Require Other Traits

You could require that all `Animal` types require `Debug` be also implemented:

```rust
trait Animal: Debug {
    fn speak(&self);
}
```

Now `Cat` won't compile until you derive (or implement) `Debug`.
