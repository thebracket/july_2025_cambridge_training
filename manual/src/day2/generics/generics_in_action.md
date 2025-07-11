# Generics in Action

There's a second format for generics that's a bit longer but more readable when you start piling on the requirements:

```rust
fn print_it<T>(x: T)
where
    T: ToString,
{
    println!("{}", x.to_string());
}
```

You can combine requirements with `+`:

```rust
fn print_it<T>(x: T)
where
    T: ToString + Debug,
{
    println!("{:?}", x);
    println!("{}", x.to_string());
}
```

You can have multiple generic types:

```rust
fn print_it<T, U>(x: T, y: U)
where
    T: ToString + Debug,
    U: ToString + Debug,
{
    println!("{:?}", x);
    println!("{}", x.to_string());
    println!("{:?}", y);
    println!("{}", y.to_string());
}
```

The generics system is almost a programming language in and of itself---you really can build most things with it.

## Traits with Generics

Some traits use generics in their implementation. The `From` trait is particularly useful, so let's take a look at it:

```rust
struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}
```

Here we've defined a type for Degrees, and a type for Radians. Then we've implemented `From` for each of them, allowing them to be converted from the other. This is a very common pattern in Rust. `From` is also one of the few surprises in `Rust`, because it *also* implements `Into` for you. So you can use any of the following:

```rust
let behind_you = Degrees(180.0);
let behind_you_radians = Radians::from(behind_you);
let behind_you_radians2: Radians = Degrees(180.0).into();
```

You can even define a function that requires that an argument be convertible to a type:

```rust
fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}
```

And you've just made it impossible to accidentally use degrees for a calculation that requires Radians. This is called a "new type" pattern, and it's a great way to add constraints to prevent bugs.

You can *also* make the `sin` function with generics:

```rust
fn sin<T: Into<Radians>>(angle: T) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}
```

The `impl` syntax is a bit newer, so you'll see the generic syntax more often.