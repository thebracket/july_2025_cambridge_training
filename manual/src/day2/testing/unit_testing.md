# Unit Testing

You've written plenty of unit tests now! Here's the basic `tmod` (testing module) setup:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        
    }
}
```

If you need crate dependencies *just for testing*, you can add them to the `dev-dependencies` section of your `Cargo.toml` file. This keeps your main dependencies clean and focused on production code. Then they won't bloat your main file.

## NexTest

Cargo NexTest is becoming a very popular testing framework. I personally stick with the built-in `cargo test`, but it's worth checking out if you're looking for more advanced features. In particiular, it works very well with CI pipelines and *huge* projects.

[cargo-nextest]: https://nexte.st/

## Where Do I Put Tests?

UNIT tests go in the same file as the code they test, inside a `#[cfg(test)]` module. This keeps your tests close to the code they test, which is great for readability and maintainability.

You *can* also put tests in a separate file, or even in the `tests` directory. I strongly recommend keeping unit tests close to the code they test.