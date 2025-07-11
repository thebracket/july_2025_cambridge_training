# Workshop: Login Library

A client/server system without authentication or authorization is a *bad idea*. So we're going to build a simple login library that can be used by both the client and the server, and a simple CLI tool to manage users.

## Task 1: Create a new library

> The code for this workshop is in `code/day1/login_library`.

### Step 1: Make the library crate and add dependencies

![](../../images/ScrollTime.png)

```bash
cargo new login_library --lib
```

In `login_library/Cargo.toml`, add the following dependencies:

```toml
[dependencies]
serde.workspace = true
serde_json.workspace = true
sha2 = "0" # We'll use this to hash passwords
```

### Step 2: Here's a Free Hashing Function!

```rust
fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}
```

### Step 3: Create the user struct and make it serializable

* Your `User` struct should have public fields for `username` and a *private* field for `password`.
* Make the struct serializable and deserializable with Serde.
* Create a `new` function (constructor) that takes a username and password, hashes the password and returns a `User`.

![](../../images/ScrollTime.png)

You should now have something like this:

```rust
use serde::{Deserialize, Serialize};

fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: hash_password(password),
        }
    }
}
```

### Step 4: Add a verify function

Add a function to the `User` struct that takes a password and returns true if it matches the stored password.

![](../../images/ScrollTime.png)

```rust
impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        self.password == hash_password(password)
    }
}
```

### Step 5: Add Tests

> Handy tip for Visual Studio Code users. Go to the bottom of your file and type `tmod`. It will offer `tmod (test module)`. Hit enter and it will create a test module for you.

If you don't have a test module, create one now. The empty default looks like this:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        
    }
}
```

Use the `assert!`, `assert_eq!` and `assert_ne!` macros to create tests.

You can run tests with `cargo test`.

Now create tests:

* Verify that running the password hashing function twice on the same password produces the same result.
* Verify that running the password hashing function on two different passwords produces different results.
* Verify that creating a user with a username and password stores the username correctly and hashes the password.
* Verify that the `verify_password` function returns true for the correct password and false for an incorrect password.

![](../../images/ScrollTime.png)

You should now have something like this:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_twice() {
        let password = "hunter2";
        let hash1 = hash_password(password);
        let hash2 = hash_password(password);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hashing_different_passwords() {
        let password1 = "hunter2";
        let password2 = "hunter3";
        let hash1 = hash_password(password1);
        let hash2 = hash_password(password2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_user_creation() {
        let user = User::new("alice", "password123");
        assert_eq!(user.username, "alice");
        assert_ne!(user.password, "password123");
    }

    #[test]
    fn test_verify_password() {
        let user = User::new("bob", "secret");
        assert!(user.verify_password("secret"));
        assert!(!user.verify_password("wrong_password"));
    }
}
```

`cargo test` should now pass.

### Step 6: Add a Login Manager

1. Add a new struct named `LoginManager` that has a single (public) field, a vector of `User` structs.
2. Add a `new` function (constructor).
    1. It should create an `std::path::Path` with the filename `users.json`.
    2. If the file exists, it should read the file and deserialize the JSON into a vector of `User` structs.
    3. If the file does not exist, it should create an empty vector.
3. Add a `save` function that serializes the vector of `User` structs to JSON and saves it to `users.json`.
4. Add an `add_user` function that takes a username and password, creates a new `User` struct and adds it to the vector.
5. Add a `verify_user` function that takes a username and password, searches the vector for a user with the given username and verifies the password. It should return `Option<User>`.
6. Add a test for the `add_user` and `verify_user` functions.

Since we haven't covered file I/O yet, here's some hints:

```rust
use std::path::Path;

let path = Path::new("users.json");
if path.exists() { .. }

let data = std::fs::read_to_string(path).expect("Failed to read users.json");
let users: Vec<User> = serde_json::from_str(&data).expect("Failed to parse users.json");

let data = serde_json::to_string_pretty(&self.users).expect("Failed to serialize users");
std::fs::write("users.json", data).expect("Failed to write users.json");

self.users.iter().find(..)
```

![](../../images/ScrollTime.png)

You should now have something like this:

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginManager {
    pub users: Vec<User>,
}

impl LoginManager {
    pub fn new() -> Self {
        let path = Path::new("users.json");
        if path.exists() {
            let data = std::fs::read_to_string(path).expect("Failed to read users.json");
            let users: Vec<User> = serde_json::from_str(&data).expect("Failed to parse users.json");
            Self { users }
        } else {
            Self { users: vec![] }
        }
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(&self.users).expect("Failed to serialize users");
        std::fs::write("users.json", data).expect("Failed to write users.json");
    }

    pub fn add_user(&mut self, username: &str, password: &str) {
        let user = User::new(username, password);
        self.users.push(user);
    }

    pub fn verify_user(&self, username: &str, password: &str) -> Option<&User> {
        self.users.iter().find(|user| user.username == username && user.verify_password(password))
    }
}
```

And a test:

```rust
#[test]
fn test_login_manager_add_and_verify() {
    let mut manager = LoginManager::new();
    manager.add_user("charlie", "mypassword");
    assert!(manager.verify_user("charlie", "mypassword").is_some());
    assert!(manager.verify_user("charlie", "wrongpassword").is_none());
    assert!(manager.verify_user("unknown", "mypassword").is_none());
}
```