# Generic Batching Map

You can make generic structs and enums, too. In fact, you've seen lots of generic `enum` types already: `Option<T>`, `Result<T, E>`. You've seen plenty of generic structs, too: `Vec<T>`, `HashMap<K,V>` etc.

Let's build a useful example. How often have you wanted to add entries to a `HashMap`, and instead of replacing whatever was there, you wanted to keep a list of *all* of the provided values that match a key.

Let's start by defining the basic type:

```rust
use std::collections::HashMap;

struct HashMapBucket<K,V>
{
    map: HashMap<K, Vec<V>>
}
```

The type contains a `HashMap`, each key (of type `K`) referencing a vector of values (of type `V`). Let's make a constructor:

```rust
impl <K,V> HashMapBucket<K,V> 
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }
}

So far, so good. Let's add an `insert` function (inside the implementation block):

```rust
fn insert(&mut self, key: K, value: V) {
    let values = self.map.entry(key).or_insert(Vec::new());
    values.push(value);
}
```

Uh oh, that shows us an error. Fortunately, the error tells us exactly what to do---the key has to support `Eq` (for comparison) and `Hash` (for hashing). Let's add those requirements to the struct:

```rust
impl <K,V> HashMapBucket<K,V> 
where K: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}
```

So now we can insert into the map and print the results:

```rust
fn main() {
    let mut my_buckets = HashMapBucket::new();
    my_buckets.insert("hello", 1);
    my_buckets.insert("hello", 2);
    my_buckets.insert("goodbye", 3);
    println!("{:#?}", my_buckets.map);
}
```

In 21 lines of code, you've implemented a type that can store multiple values for a single key. That's pretty cool. Generics are a little tricky to get used to, but they can really supercharge your productivity.
