# Workspace Dependencies

The setup we have right now works - but what if multiple crates all want to rely on `colored`? Here's what we have now:

```toml
[dependencies]
colored = "3.0.0"
test_library = { path = "test_library" }
```

The `[dependencies]` section *only* applies to the current crate - in this case, "hello world". That's fine with two crates---but once your complexity starts to tick up, you probably don't want 30 versions of the same library hanging around. You definitely don't want to have to update 30 `Cargo.toml` files when you need to upgrade!

In the *top-level* `Cargo.toml`, you can add:

```toml
[workspace.dependencies]
colored = "3.0.0"
```

Now *any* crate in the workspace can opt-in to using the workspace dependency as follows:

```toml
[dependencies]
colored.workspace = true
```

It's not quite as easy as `cargo add`, but you gain a lot of advantages:

* Shared build artifacts.
* Improved compile times.
* You *know* that your versions are the same throughout your project.
* Lower disk-space usage.
