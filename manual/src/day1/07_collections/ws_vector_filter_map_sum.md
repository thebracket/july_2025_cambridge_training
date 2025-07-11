# Quick Workshop: `filter`, `map`, `sum`

You’ve seen `for` loops, but Rust loves iterators. Let’s warm up with a classic.

Write code that:

1. Iterates over numbers 1 through 100,
2. Filters to even numbers only,
3. Doubles them,
4. Sums the result.

Try it first using a `for` loop and `if` statements if you like—and then with `filter`, `map`, and `sum`.

---

![](../../images/ScrollTime.png)

---

Example with iterators:

```rust
fn main() {
    let total: u32 = (1..=100)
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
        .sum();
    println!("Sum: {total}");
}