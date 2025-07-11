# Cargo.toml

`Cargo.toml` is your build manifest for Rust projects. It takes the place of `CMake`, `make`, `ninja` - and all of the other build-tools - as well as handling dependencies so you don't need to decide between `vcpkg`, `conan`, `hunter`, etc.

Let's look at the generated `Cargo.toml`:

```toml
[package]
name = "hello_rtx"
version = "0.1.0"
edition = "2024"

[dependencies]
```

* Your package *name* will be the name of your compiled program (with a `.exe` on the end, on Windows).
* *Version* uses semantic versioning. Major, minor, patch number. It's remarkable how many version 0 things Rust has.
* *Edition* is Rust's way of avoiding the "we can never fix the bugs!" problem from C++. C++ editions tend to be huge, bugs and unworkable specifications from decades ago still litter the language. Rust periodically issues a new edition, in which it is allowed to make changes to the language itself. For example, the 2024 edition made the word `gen` into a keyword (it will be used for generator support).
    * Helpfully, you can mix editions in your program. Unless a major CVE arises, old syntax will continue to compile.
    * Crates with different editions link together just fine.

The `[dependencies]` section lists the crates on which you depend.

## Let's Add a Dependency

Let's make "Hello, world!" print in green. There's a handy crate named `colored` that makes ANSI terminal colors easy to use.

You can query the available crates on [crates.io](https://crates.io), or you can use Cargo to search:

```
cargo search colored
colored = "3.0.0"           # The most simple way to add colors in your terminal
```

There are (at least!) two ways to add the dependency. In your terminal (make sure you're still in your "hello world" example directory) Let's use the easy one:

```bash
cargo add colored
```

Your `Cargo.toml` dependencies section updates to read:

```toml
[dependencies]
colored = "3.0.0"
```

> The other way is to simply add that line to `Cargo.toml`.

Now let's edit `src/main.rs`:

```rust
use colored::Colorize;

fn main() {
    println!("{}", "Hello, world!".green());
}
```

Notice:

* The crate name acts as a *namespace*. Everything it exports is in `colored::*`.
* `use` brings namespace items into the current namespace - just like `using` in C++, or `import` in Python.
* `println!` is now a little more complicated (and should look familiar to the Pythonistas!)
    * We have a *placeholder*, `{}`.
    * The placeholder is populated by additional arguments to `println!`.
    * We've added `.green()` to the string. That's what the colorize trait does.


Run the program now, and you have a green "hello world".

## Dependency Dependency

You have a lot of flexibility in specifying dependencies. We'll take a deeper dive later on in this section. But if you're anything like me, when I came over to Rust from daily life as a C++ programmer my thought process was:

* I can't possibly audit all of these dependencies.
* What about `left-pad`?
* Doesn't this make my code fragile?

Truthfully: I can't possibly audit all of these, and there's nothing to stop someone from putting up left-pad. This is especially true because dependencies can pull in more dependencies. It's even possible to have multiple versions of the same dependency in your build tree. That puts some responsibilty on you, the developer: 

* Use dependencies judiciously.
* Visit [blessed.rs](https://blessed.rs) for crate recommendations.
* Don't create - or rely on - JavaScript/NPM style "left_pad" (it's built in), "is_true" (and the one-line "is_false").
* Run `cargo tree` (Cargo really does everything) after you add a dependency to make sure you didn't just start depending upon the entire planet!
