# Rust in an Elevator

Graydon Hoare likes to tell people that Rust came about because of a broken elevator.  
The Mozilla office elevator kept failing, and after multiple vendor repair attempts, the root cause turned out to be a firmware bug — in some badly written C.

Climbing several flights of stairs every day was… motivating. Enough to inspire a new systems language.

More realistically, Graydon — and friends — were working on **Firefox**, which was *riddled* with the kinds of issues C and C++ make all too easy.

Early Rust design focused on preventing:

- Data races (unsynchronized access to shared memory)
- Buffer overruns/overflows (especially ones that silently corrupt memory)
- Unchecked numeric overflows
- Null pointer dereferences
- Use-after-free errors
- Build systems that require a Ph.D.

Rust isn’t perfect, but it’s an evolution. Many of its ideas come from C++ — **with the defaults flipped**.

In C++, you *can* write safe code… but it’s easy to forget a `nullptr` check or accidentally bypass RAII.  
Rust does the opposite:  
- **Safe is the default**  
- **Nulls are replaced with `Option<T>`**  
- **Ownership and borrowing rules are enforced at compile time**

> Many developers who learn Rust find that even when they go back to other languages, they think a little more “Rustily” — and catch more bugs before they happen.
