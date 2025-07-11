# Workshop: Making a Project

Let's all fire up a terminal. Create and navigate to a scratch directory (I'm using `rtx`). Then we'll use `cargo new` to create a new project.

```bash
mkdir rtx
cd rtx
cargo new hello_rtx
cd hello_rtx
```

Now open the `hello_rtx` directory in your editor of choice.

You'll see the following files:

```
.
├── Cargo.toml (Build Manifest)
└── src (directory for source code)
    └── main.rs (main program source code)
```

Open `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

It's pretty self-explanatory. A `main` function, just like every other language. A print command that sends "Hello, world!" to the terminal. You've probably seen this a hundred times in different languages.

> If you haven't seen enough "hello world", this page should satisfy you: [http://helloworldcollection.de/](http://helloworldcollection.de/)

Now type:

```bash
cargo run
```

You should see the expected output: "Hello, world!"

You could even type:

```bash
cargo run --release
```

And now you see "Hello, world!" even faster! That's how you tell the Rust toolchain to enable optimizations. By default, Rust will build in "debug" mode with a lot of extra checks in place.

Now run `ls -ah` (or Windows equivalent). 

```
ls -a
.  ..  Cargo.toml  .git  .gitignore  src
```

A `.git` directory and `.gitignore` file have appeared! By default, Cargo will make a new Git repository (if it doesn't detect that you are already inside one). You can turn this off with `cargo new --vcs none my_project_name`. Cargo also has native support for `hg`, `pijul`, and `fossil`.