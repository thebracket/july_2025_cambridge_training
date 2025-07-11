# Move by Default

This trips up *everyone* who starts with Rust.

```rust
fn do_something(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello");
    do_something(s);
    println!("{}", s);
}
```

Try it - it won't compile. The error message seems to get longer and longer every time I teach it. The gist is "you moved s". What does that mean?

1. `String` allocates memory on the heap (it's a *lot* like a C++ `std::string` with extra UTF-8 validation). 
2. When you declare `let s = String::from("Hello");`, Rust allocates memory for the string, and then `s` becomes the owner of that memory.
3. When you pass `s` to `do_something`, Rust moves the ownership of that memory to the function. This is called a *move*, and Rust is *move by default*.
4. Ownership was moved to the function - it now belongs to that function. When the funciton exists, `s` is "dropped" (freed) and the memory is returned to the OS. (This is RAII - Resource Acquisition Is Initialization).
5. "Use after free" and "Use after move" are both really common bugs in the C++ world. Rust has *destructive moves* - once you move something, you can't use it again. This is a compile-time error, not a runtime error.

> Pop Quiz: Who wants to explain what `std::move` in C++ *actually* does?

Rust is saving you from a common issue in C++:

```cpp
#include <iostream>
#include <string>

void do_something(std::string s) {
    std::cout << s << "\n";
}

int main() {
    std::string s = "Hello";
    do_something(std::move(s)); // Cast to rvalue, but doesn't move
    std::cout << s << "\n"; // Oops, use after move! "s" is in a valid but unspecified state
    return 0;
}
```

> Answer: It casts the object to an rvalue, which allows you to move it. It doesn't actually move anything. This is a common misconception.

## Except Where You Copy!

Rust has a few types that implement the `Copy` trait implicitly. These are typically primitives (`i32`, `bool`, etc) and types that are made up of primitives (tuples of primitives, for example). You can always implement `Copy` for your own types, but it's not common.

```rust
fn do_something(i: i32) {
    println!("{}", i);
}

fn main() {
    let i = 5;
    do_something(i);
    println!("{}", i); // This works, because i32 is Copy
}
```

This is a little confusing, but it's a reflection of the underlying CPU. Most of the time, primitives will be passed in registers, and the CPU doesn't care if you copy a register. It's also very fast to copy a few bytes of memory.

## Moving Back and Forth

You can move variables into functions, and move them back out again. This is a little clunky, but it works:

```rust
fn do_something(s: String) -> String {
    println!("{}", s);
    s // Return ownership back to the caller
}

fn main() {
    let s = String::from("Hello");
    let s = do_something(s);
    println!("{}", s);
}
```

Your code compiles! It's *ugly*, but it works. This is a common pattern in many functional programming languages, and it's a good way to think about ownership. You have the same "Return Value Optimization" (RVO) that C++ has, so the compiler is smart enough to avoid a copy here.