# Dependency Management in Practice

For any serious project, don't `cargo search <what I need to do>` and install the first hit. Instead, visit `blessed.rs` and see if there's a recommended crate for your use case. If not, check crates.io, and the project's GitHub. Check the license, check what it depends on, check if its maintained. We *don't* want to wind up in JavaScript dependency land!

## Pin Your Dependencies

When you ship a version, pin your dependencies. This is done automatically by Cargo when you use a `Cargo.lock` file. Set `=` versions in your `Cargo.toml` if you want to be extra sure. This avoids accidental breakage when a dependency releases a new version.

### Vendoring Dependencies

The command `cargo vendor` will copy all your dependencies into a `vendor/` directory. You can then check this into your source control, and use `source = "vendor"` in your `.cargo/config.toml` to make sure you always build from the vendored dependencies. This is especially useful for production builds, or if you need to build offline.

If you are in an environment that requires a "known good, hermetic, auditable build", vendoring is a must.

## Audit Your Dependencies

Use `cargo audit` or `cargo deny` to check your dependencies for known vulnerabilities. This is especially important if you're writing security sensitive code, or code that handles sensitive data.

`cargo audit` (`cargo install cargo-audit`) checks your `Cargo.lock` against a database of known vulnerabilities. `cargo deny` ( `cargo install cargo-deny`) is more powerful, and can also check licenses, and enforce policies on your dependencies.

> You may *have* to use `cargo deny` with a carefully curated denylist in order to satisfy your legal department!