# Quick Workshop: Your Own FizzBuzz

Let's put our knowledge to work. Here's a simple Python `FizzBuzz` test:

```python3
for i in range(1,20):
  if i % 3 == 0 and i % 5 == 0:
    print("Fizzbuzz")
  elif i % 3 == 0:
    print("Fizz")
  elif i % 5 == 0:
    print("Buzz")
  else:
    print("{}".format(i))
```

Using the control flow and variable knowledge we've covered, go ahead and make a Rust version.

![](../../images/ScrollTime.png)

Answer:

```rust
fn main() {
    for i in 1 .. 20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("Fizzbuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
```