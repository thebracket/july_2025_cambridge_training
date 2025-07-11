# println! - there's nothing wrong with it

I just want to say that `println!` debugging is *not* a bad thing. It's great, I use it all the time for quick debugging. By the time your application reaches production, you should have a more robust logging solution in place, but for development and debugging, `println!` is perfectly fine.

It's worth noting that `println!` uses another macro called `format!` under the hood, which is used to format the string before printing it. This means you can use all the formatting features of `format!` with `println!`.

`format!` has a *lot* of features. The full documentation is [here](https://doc.rust-lang.org/std/fmt/index.html).

Most of the logging frameworks *also* use `format!`!