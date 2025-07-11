# Types

Rust is a strongly typed language, and is very opinionated---and precise--about types.

## Primitives

In C (and C++), you may be used to types such as `bool`, `short`, `signed short`, `int`, `long`, `long long`, `float`, `double` or even `long double`. The exact size of these varies by platform (although modern chips outside of the embedded world have largely standardized on meanings). Many places have adopted `stdint.h` to have more precise definitions `int8_t`, `uint32_t`, etc.

Rust specifies the precision in types:

* `u8`, `u16`, `u32`, `u64`, `u128`
* `i8`, `i16`, `i32`, `i64`, `i128`
* `f32`, `f64`

## Conversion

In C, you can get away with (but linters and many compilers will issue a warning!):

```c
long a = 257;
short b = a;
```

Rust is a bit more pedantic (a good thing - it's helping you avoid data loss).

The safest way to convert between types is to use `into()`. For example:

```rust
fn main() {
    let i: u8 = 64;
    let j: u16 = i.into();
    println!("{i} {j}");
}
```

Into is *only* defined for conversions that are guaranteed to be safe. For example, this will not compile:

```rust
fn main() {
    let i: u16 = 64;
    let j: u8 = i.into();
    println!("{i} {j}");
}
```

You can use `try_into` to convert and receive an error if the conversion will lose data:

```rust
fn main() {
    let i: u16 = 64;
    let j: u8 = i.try_into().unwrap_or_else(|_| {
        println!("Conversion Failed");
        0
    });
    println!("{i} {j}");

    let i: u16 = 257;
    let j: u8 = i.try_into().unwrap_or_else(|_| {
        println!("Conversion Failed");
        0
    });
    println!("{i} {j}");
}
```

> We're using error handling syntax and a closure here - we'll talk about those in a bit!

You can *also* use `as`. `as` is dangerous, because it works just like casting in C and C++:

```rust
fn main() {
    let i: u16 = 64;
    let j = i as u8;
    println!("{i} {j}");

    let i: u16 = 257;
    let j = i as u8;
    println!("{i} {j}");
}
```

So keep `as` for when you *need* it - using `into` and `try_into` can help avoid common classes of bugs.

> TL;DR: Use `into` or `try_into` where possible. Reach for `as` only when you really mean it.

## Overflow

This is one of the few cases in which `debug` (`cargo run`) and `release` (`cargo run --release`) behave differently.

```rust
fn main() {
    let mut i: u8 = 250;
    let mut count = 0;
    while count < 10 {
        i += 1;
        println!("{i}");
        count += 1;
    }
}
```

In debug mode: this panics. In release mode, `i` overflows and wraps around as you'd expect in most other languages. This can help catch bugs (run in debug mode occasionally!), but it's better to specify your intent.

```rust
fn main() {
    let mut i: u8 = 250;
    let mut count = 0;
    while count < 10 {
        i = i.wrapping_add(1);
        println!("{i}");
        count += 1;
    }
}
```

You can use `checked_add`, `saturating_add`, and `overflowing_add` as well. See the [documentation](https://doc.rust-lang.org/std/primitive.u8.html#method.checked_add) for more.

This is really useful for "future you" or other people you work with: your intent is clear.