# Rayon - Chunking Made Easy

I know you just worked hard on chunking workloads manually. That's great, but it's a lot of work, and it's easy to get wrong. Rayon is a library designed to make parallelism easy (it's not free: it'll make one thread per core with work-stealing).

```rust
use rayon::prelude::*;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let primes: Vec<usize> = (0..1_000).into_par_iter() // Rayon parallel iterator
        .filter(|&n| is_prime(n)) // Filter primes
        .collect(); // Collect into a vector
    for prime in primes {
        println!("Found prime: {}", prime);
    }
}
```

To use Rayon, add this to your `Cargo.toml`:

```toml
[dependencies]
rayon = "1.9"  # Or whatever the latest version is
```