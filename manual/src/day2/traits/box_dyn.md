# Boxed Dynamic - Vector of Traits

So now we can make animals that share a treat. So how do we combine them into a vector? If you're used to other languages, you might think you can just do this:

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let animals: Vec<Animal> = vec![Dog, Cat];
}
```

That doesn't compile! The compiler helpfully suggests that you need `dyn`, and doesn't really tell you much more.

## Boxed Dynamic Trait Objects

To use a trait as a type, you need to use a *trait object*. This is done by using `dyn` before the trait name, and you need to put it inside a `Box` if you want to store it in a vector. Here's how you can do that:

```rust
trait Animal {
    fn speak(&self);
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        animal.speak();
    }
}
```

The syntax here isn't great (Rust isn't an Object Oriented Programming language, so it doesn't have a lot of syntactic sugar for this), but it works.

The `Box` is a smart pointer. It's basically `unique_ptr` from C++. It has single ownership, and can be moved around - but you can't clone it. The `dyn` keyword indicates that this is a dynamic trait object, meaning that the type of the object is determined at runtime.

You are making a vector of pointers, each of which is stored on the heap. This allows you to store different types that implement the same trait in the same vector --- but it can also lead to some performance overhead due to dynamic dispatch (generally TINY unless you have a LOT of them).