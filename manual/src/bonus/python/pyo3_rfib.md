# Let's Add a Rust Module

`PyO3` is a Rust crate designed to make it easy to write code in Rust, and make it available in Python. This is surprisingly common - every time you use something like `numpy` you are actually calling into code written in another language (C++ in `numpy`'s case). Python is *great* for fast prototyping, and is a great glue for connecting modules that have been upgraded for performance reasons once you know they work.

Creating a Python module requires a little bit of setup. In `Cargo.toml`, you need to edit the following:

```toml
[dependencies]
pyo3 = "0"

[lib]
name = "rfib"
crate-type = ["cdylib"]

# Extra flags to make the Mac happy
[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

This tells Rust to compile a dynamic library. On Windows you get a `.dll` file. On Mac, `.dynlib` file. On Linux, a `.so` file. If you are using Windows or Mac, you will need to rename the output to `.so`---that's what Python expects to receive.

Next, you write the actual Rust:

```rust
fn fibo(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}
```

And then you need to make it available to Python by "wrapping" it:

```rust
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

#[pyfunction]
fn recur_fibo(n: u64) -> PyResult<u64> {
    Ok(fibo(n))
}

#[pymodule]
fn librfib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(recur_fibo))?;
    Ok(())
}
```

Finally, you can build it with `cargo build --release` and copy the `librfib.so` file to your Python directory. Then in the Python code, you can import all the `#[pyfunction]` marked functions (that have been exported) and call them directly:

```python
#/usr/bin/python3
import time
from librfib import recur_fibo

print("Single thread")
results = []
t0 = time.time()
for i in range(40):
    results.append(recur_fibo(i))
t1 = time.time()

print(results)
print("Time: ", t1-t0)
```

## But Wait, Let's Go Faster!

You don't have to limit yourself to a single thread on the Rust side. In `code/python/rfib` we've integrated the Rayon version as well.

On my workstation, timing are as follows:

```
Single thread
[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155]
Time:  0.2625265121459961


Multi thread
[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155]
Time:  0.10398244857788086
```

So intead of nearly 12 seconds, we've improved performance to 0.26 seconds single-thread, and 0.10 seconds by using Rayon!