# FxHash

Rust's `HashMap` uses a cryptographically secure hashing algorithm by default. This is great for security, but it can be slow for some workloads. You can substitute the hashing algorithm if you need to.

The Rust compiler uses a fast, non-cryptographic hashing algorithm called FxHash for its internal data structures. You can use this in your own code by adding the `fxhash` crate to your `Cargo.toml`:

```toml
[dependencies]
fxhash = "0.2"
```

Then, you can use it like this:

```rust
use fxhash::FxHashMap;

fn main() {
    let mut map: FxHashMap<&str, i32> = FxHashMap::default();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
}
```

You can also use `FxHashSet` if you just need a set of values:

```rust
use fxhash::FxHashSet;

fn main() {
    let mut set: FxHashSet<&str> = FxHashSet::default();
    set.insert("one");
    set.insert("two");
    set.insert("three");

    for v in &set {
        println!("{}", v);
    }
}
```
