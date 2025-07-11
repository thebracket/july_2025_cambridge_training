# Workshop: Calculating Primes Inefficiently On All Cores

> My answer is in `code/day2/ws_primes.rs`.

Your goal is to make a program that iterates over all numbers from 0 to 100,000 and prints all the prime numbers it finds.

We're going to use a really inefficient function:

```rust
fn is_prime(n: usize) {
    if n <= 1 {
        return;
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return;
            }
        }
        println!("Found prime: {}", n);
        return;
    }
}
```

As a hint, you can determine the number of cores on your machine with:

```rust
let num_cores = std::thread::available_parallelism().unwrap().get();
```
