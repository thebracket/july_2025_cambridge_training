# Project Layout

"How do we lay out a Rust project?" is a common question - and sadly the answer always boils down to "it depends upon what you're trying to do". There are some common patterns and practices that will help you - so we'll go over those.

## Self-Contained Services

When you're starting out, it's often easiest to create a single binary crate. Make a module (in a directory) for each major component of your application. Treat the interior of the module as private, and expose a public API for the module.

### If Compile Time Becomes a Problem - or You Have Multiple Teams/Binaries

Take the self-contained modules, and turn them into library crates. Since you already have a public API, this is usually a straightforward process (and you quickly find where you accidentally coupled modules together!).

### If You Need to Scale Out

Add a `main`.rs to each service, add the skeleton to run it - and you have a microservice! (You can have both a `main.rs` and a `lib.rs` in the same crate).

## Re-Exporting

Library crates that rely heavily on other crates often re-export those crates. You can re-export entire crates with `pub use`:

```rust
pub use serde::{Serialize, Deserialize};
```

This allows users of your crate to just depend on your crate, and get access to the re-exported crates. This is especially useful for crates that provide a framework or API - you can re-export the common dependencies so users don't have to manage them.