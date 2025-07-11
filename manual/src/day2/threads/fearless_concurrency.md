# What is Fearless Concurrency?

Rust often advertises "Fearless Concurrency".

That's a little brave, because concurrent code is stil hard! Rust's ownership model helps you write concurrent code that is safe, in particular by preventing data races.

It's still not perfect - you have to be careful of deadlocks, and "logical races" (that is when the data is perfectly valid, but you get the wrong answer because two threads are racing to update the same data).