# Flow Control

## Looping

You've seen a `while` loop, and you're familiar with loops in other languages. So we'll keep this very brief.

```rust
fn main() {
    let mut x = 0;
    // loop
    loop {
        x += 1;
        if x == 10 {
            break;
        }
    }

    // loop with a label
    x = 0;
    'outer: loop {
        x += 1;
        if x == 20 {
            break 'outer;
        }
    }
    
    // while
    x = 0;
    while x < 30 {
        x += 1;
    }

    // for
    for x in 0..40 {
        // do something
    }

    // inclusive range
    for x in 0..=40 {
        // do something
    }
}
```

Nothing revolutionary there. One note: you can't use `for` in a `const` context - because it desugars to an iterator, and iterators are not `const`.

## Scope Return Values

In Rust, the value of a scope block is the value of the last expression in that block. This is perfectly valid:

```rust
fn main() {
    let x = {
        let mut y = 0;
        y += 1;
        y
    };
    println!("x is {x}");
}
```

There's also a unit type `()` for indicating the absence of a value. So this is also valid:

```rust
fn main() {
    let x = {
        let mut y = 0;
        y += 1;
    };
    println!("x is {x:?}");
}
```

## Conditionals

Rust `if` statements are very similar to other languages. The only real difference is that `if` statements are expressions, so they return a value:

```rust
fn main() {
    // A normal if statement
    let x = 5;
    if x == 5 {
        println!("x is five");
    }

    // Returning with if
    let y = if x == 5 {
        10
    } else {
        20
    };
    println!("y is {y}");
}
```

> There are no ternary operators in Rust.