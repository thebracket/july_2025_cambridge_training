# Iterating

So far, we've iterated vectors using a `for` loop (optionally with `enumerate()` to get the index). This is common (and a great idea if you need to modify elements by index), but there are other ways to iterate.

These are equivalent (and compile to the same code):

```rust
fn main() {
    let v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }
    for i in v.iter() {
        println!("{}", i);
    }
    v.iter().for_each(|i| println!("{}", i));
}
```

There are benefits and drawbacks to each approach:
* `for` loops are simple and easy to read.
* Iterator functions typically invoke a closure, which:
    * Can be hard to read if you're not used to them.
    * Can be inlined by the compiler, which can make them faster.
    * Make early returns (`break`, `continue`, `return`) harder to express.
* Iterator functions can be chained together to create complex iteration pipelines.
* Iterator functions can be lazy, which can save memory and CPU cycles.

## Some Common Iterator Functions

```rust
fn main() {
    let v = vec![1, 2, 3];

    // Summing
    let sum: i32 = v.iter().sum();

    // Filtering and collecting
    let even: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let odd: Vec<i32> = v.iter().filter(|&&x| x % 2 != 0).copied().collect();

    // Mapping and collecting
    let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();

    // `collect`, `foreach` are terminal operations. They consume the iterator.
    // `filter`, `map` are lazy operations. They return a new iterator.
}
```

There are *many* iterator functions. You can find a list in the [standard library documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html). The `Itertools` crate on crates.io has even more.

Here's an example using Itertools to sort, deduplicate with a counter, and print the results:

```rust
use itertools::Itertools;

fn main() {
    let v = vec![1, 2, 3, 1, 2, 3, 4, 5, 6];
    v.iter()
        .sorted()
        .dedup_with_count()
        .for_each(|(count, value)| println!("Value: {}: Count {}", value, count));
}
```


> There's another added benefit to using iterator functions: you can use a library called `Rayon` to automatically parallelize your iterator pipelines. We'll talk about that later.