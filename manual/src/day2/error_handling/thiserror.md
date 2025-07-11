# ThisError

Defining a full error type in Rust is a bit of a pain. You need to define a struct, implement the `Error` trait, and then implement the `Display` trait. You'll learn about traits next week, but for now you can think of them as "interfaces" that define what a type can do.

```rust
#[derive(Debug, Clone)]
enum UsersError {
    NoUsers, TooManyUsers
}

use std::fmt;

impl fmt::Display for UsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UsersError::NoUsers => write!(f, "no users found"),
            UsersError::TooManyUsers => write!(f, "too many users found"),
        }
    }
}
```

That's quite a lot of typing for an error! Pretty much nobody in the Rust world does this, unless you are in an environment in which you can't rely on external crates. Let's do the same thing with the `thiserror` crate:

```bash
cargo add thiserror
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers, 
    #[error("Too many users were found")]
    TooManyUsers
}
```