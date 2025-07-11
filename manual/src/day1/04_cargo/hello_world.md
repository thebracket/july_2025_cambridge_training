# Hello World: Not The Best Place to Start

We mostly setup and ran Hello World to make sure that everyone has a working setup. From a teaching point of view, I always grumble a little that Hello World isn't the greatest place to start:

```rust
fn main()
```

Fair enough - main functions are pretty normal.

```rust
{

}
```

Great - we have curly bracket scopes (sorry, Python lovers - I'm a fan of brackets!).

```rust
println!("Hello, world!");
```

Lines ending in semicolons - great. But why is `println!` surprising? It's a *macro*. Macros are a little on the advanced side, and this is a *compiler supported macro* meaning that not only can it bypass Rust's syntax rules - it can do a few things that regular macros struggle with, too!

We'll talk a little bit about macros later. For now - just pretend its a function call. We'll run into a few of those in a moment with things like:

```rust
#[derive(Clone, Debug)]
```

That's some great helper code - it implements the `Clone` and `Debug` traits and makes programs work. Until we get to traits - don't worry too much.