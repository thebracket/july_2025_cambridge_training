# Cargo Workspaces

> When I was working on Hands-on Rust I forgot to create a workspace, and found myself with over 100 gigabytes of build files lying around.

Have a look in your `hello_rtx` directory after you've run/compiled the program. There's a `target` directory. This is where build artifacts, including your executable file. Even with just the one dependency, there can be a lot in there:

```bash
cd target
tree
```

When you're working on a big project, with lots of related crates, your `target` directory can become really excessive. Workspaces exist to solve this problem (and a few others):

* Build artifacts are shared in a single, workspace-wide `target` directory.
* You can share dependencies between workspace members, keeping versions the same.

## Let's Create a Workspace

The remaining workshops and content in this class all assume that you're in a workspace. The example files in this repository are all in a workspace, too.

### Clean Up

Let's start by cleaning up. In your `hello_rtx` directory, run:

```bash
cargo clean
```

Cleaning erases everything in the `target` directory.

### Create a Workspace

Now open `Cargo.toml` and add a section (section order doesn't matter, but at the bottom makes sense):

```toml
[workspace]
members = []
```

### Add a Test Library

Now make sure you are still in your `hello_rtx` directory. Let's add a new project. This time, we'll make a library.

```bash
cargo new test_library --lib
```

Using `--lib` tells Rust that you want to make a library, not an executable.

Now your directory looks like this:

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── test_library
    ├── Cargo.toml
    └── src
        └── lib.rs
```

Look in your `Cargo.toml`, and your library has been added as a workspace member:

```toml
[workspace]
members = ["test_library"]
```

Also notice that the `test_library` has its *own* `Cargo.toml` --- not everything in a workspace has to share dependencies.

`cargo new` has made a `lib.rs` file instead of a `main.rs` file.

* `lib.rs` compiles as library code.
* `main.rs` compiles as executable code.

> You can have both in the same project, and `main.rs` can refer to the library in `lib.rs`. This is a common pattern for microservices: write the service in `lib.rs` (and other files), and use `main.rs` to provide access functions.

So what has Cargo created for the default library code? Here's `lib.rs`:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

The library contains a function, with the `pub` keyword. `pub` means "public" - it is *exported* from the library. If you omit the `pub`, other crates *cannot* call the function.

We have a whole section on unit tests - but the *short* version is the library includes an example unit test that will only be compiled when you run `cargo test`.

## Let's Use The Library

Open your `Cargo.toml` and add a dependency on the workspace library:

```toml
[dependencies]
colored = "3.0.0"
test_library = { path = "test_library" }
```

Now edit `src/main.rs` to use it:

```rust
use colored::Colorize;
use test_library::add;

fn main() {
    println!("{}", "Hello, world!".green());
    println!("{}", add(1, 2));
}
```

If you type `cargo run`, you will see:

```
Hello, world!
3
```

And if you look at your directory tree, there's just the one `target` directory.