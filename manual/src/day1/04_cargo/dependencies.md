# More About Dependencies

You can depend on local projects:

```toml
[dependencies]
my_library = { path = "../my_library" }
```

You can depend upon crates in git (there's an optional `branch` specifier, too):

```toml
[dependencies]
my_library = { git = "https://github.com/amethyst/bracket-lib.git" }
```

You can lock to a specific version:

```toml
[dependencies]
colored = "=3.0.0"
```

Or allow anything compatible with a version:

```toml
[dependencies]
colored = "^3.0.0"
```

Finally, a really handy trick. If you need to work on the road - or guaranty a repeatable build, run:

```bash
cargo vendor
```

It will download all of your dependencies, and tell you what to add to `Cargo.toml` to use the local copy. That's my solution to unreliable wifi when I give workshops at conferences!

We'll cover more as we get to it, rather than reading the Cargo book together!