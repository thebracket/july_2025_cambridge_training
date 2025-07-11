# Enums

> I thought every language had enums until I went to GopherCon. That was embarrassing.

Rust enumerations are a very powerful feature. You can use them like a regular C/Java enum:

```rust
    enum Color {
        Red,
        Green,
        Blue,
    }

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
```

> In this case, `match` is basically `switch` from other languages.

Note that `match` is exhaustive - you must cover every case or provide a default (`_ => ...`). This won't compile:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
    }
}
```

This will compile:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        _ => println!("Not red"),
    }
}
```

> This is a great feature when you are refactoring. You add a new enum value, and the compiler tells you everywhere you need to handle it.