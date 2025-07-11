# Integration Testing

While unit tests are meant to test individual components in isolation, integration tests are designed to test how different parts of your code work together. In Rust, integration tests are typically placed in the `tests` directory at the root of your project.

## Setting Up Integration Tests

Have a look at the `tester` project in the `code/day2/tester` directory. It has a simple function that adds two numbers together, and an integration test that checks if this function works correctly.

Here's the structure of the `tester` project:

```
tester
├── src
│   └── lib.rs
└── tests
    └── integration-test.rs
```

In `lib.rs`, we have the following code:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

And in `integration-test.rs`, we have:

```rust
use tester::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

Obviously, that's not a realistic test setup - but it shows you how you *should* organize your tests.

We'll revisit tests when we get to `async` code.