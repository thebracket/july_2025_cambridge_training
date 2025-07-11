# Anyhow

There's a crate named `anyhow` that makes it easy to box errors. Add it to your project with:

```bash
cargo add anyhow
```

Then you can replace the `Box` definition with `anyhow::Error`:

```rust
fn anyhow_load_users() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
```

It still functions the same way:

```
Error: The system cannot find the file specified. (os error 2)
```

In fact, `anyhow` is mostly just a convenience wrapper around `Box` and `dyn`. But it's a *very* convenient wrapper!

Anyhow does make it a little easier to return your own error:

```rust
#[allow(dead_code)]
fn anyhow_load_users2() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    if users.is_empty() {
        anyhow::bail!("No users found");
    }
    if users.len() > 10 {
        return Err(anyhow::Error::msg("Too many users"));
    }
    Ok(users)
}
```

I've included the short-way and the long-way - they do the same thing. `bail!` is a handy macro for "error out with this message". If you miss Go-like "send any error you like", `anyhow` has your back!

> As a rule of thumb: `anyhow` is great in client code, or code where you don't really care *what* went wrong---you care that an error occurred and should be reported.