# RustUp

> The [RustUp Book](https://rust-lang.github.io/rustup/index.html) documents the RustUp tool in *detail*. This page is here for your reference later. We'll look at verifying your installation and listing your installed toolchains in class.

Installing Rust gives you a command: `rustup`. `rustup` is responsible for installing, updating and removing Rust toolchains and tools.

## Everyone: Verify Your Install!

Type the following into a terminal:

```bash
rustup --version
```

If you *don't* see something like `rustup 1.28.2 (e4f3ad6f8 2025-04-28)`, then either:

* You *just* installed this right before class started, and missed the "restart your terminal or run this source command" note (don't worry, we've all been there)
* You installed in a different way - and should probably head to [https://rustup.rs](https://rustup.rs)!

## Toolchains

Rust can cross-compile to a lot of different platforms. On my workstation, I can use `rustup show` to see:

```
rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/herbert/.rustup

installed toolchains
--------------------
stable-x86_64-unknown-linux-gnu (active, default)
nightly-x86_64-unknown-linux-gnu
nightly-2023-06-21-x86_64-unknown-linux-gnu
1.80.0-x86_64-unknown-linux-gnu
1.83.0-x86_64-unknown-linux-gnu

active toolchain
----------------
name: stable-x86_64-unknown-linux-gnu
active because: it's the default toolchain
installed targets:
  wasm32-unknown-unknown
  x86_64-unknown-linux-gnu
```

You'll see different targets, depending upon the platform you're running.

## Update Rust

> Don't do this now! Updating live in class is asking for gremlins!

At any time, you can update to the latest Rust version with:

```
rustup update
```

## Other Toolchains

You can control which toolchains are installed with:

```
rustup toolchain list - Lists the installed toolchains
rustup toolchain install - Add a toolchain
rustup toolchain uninstall - Remove a toolchain
```

For example, to add WASM support:

```bash
rustup toolchain install wasm32-unknown-unknown
```

> The list of supported platforms changes regularly. You can usually find it at: [https://doc.rust-lang.org/rustc/platform-support.html](https://doc.rust-lang.org/rustc/platform-support.html)
