# Unsafe: What It Is and Isn’t

Rust has an `unsafe` tag. It's not a "you can do anything" tag (some things like aliasing will still not work!), but it does allow you to do things that Rust normally prevents.

`unsafe` does not mean "BAD" or even "THIS IS DANGEROUS". It means "The Rust compiler cannot guarantee that this code is safe, so you must ensure it is safe yourself."

There really are three types of `unsafe` code:

1. "I have to do this" - if you're calling code (via FFI), it's automatically considered unsafe because the Rust compiler cannot verify the safety of the foreign code.
2. "I need this speed boost" - you have to be really careful here! Skipping bounds-checking *can* give you a speed boost, it can also lead to terrible things happening. Benchmark to prove that it helps, AND that you really need it. Then document your unsafe code in detail, and expect someone to complain in code review.
3. "Leeroooy Jenkins!" - this is the "I don't care about safety, I just want to do it" code. This is the most dangerous, and you should avoid it unless you really know what you're doing.
