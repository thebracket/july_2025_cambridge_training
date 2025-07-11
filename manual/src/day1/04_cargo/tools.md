# Useful Cargo Tools

## Project Management

* `cargo new` and `cargo init` make new projects (`init` in the current directory, `new` in a new directory with the project name)

## Build Tools

* `cargo check` does syntax compilation only, without building object files. Useful for a *quick* "does it build?" check.
* `cargo build` compiles and links.
* `cargo run` compiles, links and runs.
* `cargo test` runs any unit tests you've defined.
* `cargo bench` runs any benchmarks you've included.
* `cargo clean` cleans up your `target` directory.

## Linting

* `cargo clippy` runs a HUGE list of best-practices. Run Clippy often. You can enable it by default in most editors.
* `cargo fix` will try and apply Clippy's recommentations, where possible. *Use with care — especially in large projects — as it rewrites code automatically.*

## Documentation

* `cargo doc` will build a documentation tree in HTML/JS.
* `cargo doc --open` will build it and open it in a browser.
* `cargo doc --lib --open` is what I usually use; only document my library (not all dependencies as well)!

## Dependencies

* `cargo update` will try and update dependencies for you, while maintaining "pinned" versions.
* `cargo search` finds dependencies on `crates.io` (or your private registry).
* `cargo add` adds dependencies.

## cargo tree

`cargo tree` will show you a tree of all of your dependencies, helping you find out which crate depends upon what other crate.