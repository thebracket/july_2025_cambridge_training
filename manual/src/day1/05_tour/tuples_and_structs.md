# Types - Tuples and Structs

## Tuples

Most languages have a form of tuple. Rust has them as a first-class citizen, including destructuring.

> Don't go tuple crazy. If you have more than 3 or 4 elements, you probably want a struct.

```rust
fn main() {
    // Define a tuple
    let tuple = (1, "hello", 4.5);
    // Access by index
    println!("First: {}, Second: {}, Third: {}", tuple.0, tuple.1, tuple.2);
    // Destructure
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);
    // Ignore elements
    let (x, _, z) = tuple;
    println!("x: {}, z: {}", x, z);
}
```

## Structs

Structures take the role of both C-style `struct` elements and classes in C++, Python and Java. They are a way to group related data together.

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("Name: {}, Age: {}", person.name, person.age);
}
```

You can define associated functions on structs:

```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let person = Person::new("Bob", 25);
    println!("Name: {}, Age: {}", person.name, person.age);
}
```

> Notice that `new` is just a convention. Rust doesn't have constructors --- it's just a function, using Person as a namespace. Note that `Self` is an alias for the type of the struct.

You can also define methods that take `self` as the first argument:

```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut person = Person::new("Charlie", 40);
    println!("Before birthday: {}, Age: {}", person.name, person.age);
    person.birthday();
    println!("After birthday: {}, Age: {}", person.name, person.age);
}
```

> We'll worry about `&mut` in a bit. Any method that includes some variant of `self` as the first argument is called a method. Just like `self` in Python or `this` in C++, the method implicitly receives the instance as its first argument — but you must write it explicitly (`self`, `&self`, or `&mut self`) in Rust.

Rust doesn't have inheritance. You'll never find a struct inheriting from another struct. Instead, Rust uses composition and traits to achieve polymorphism.