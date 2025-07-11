# Borrowing

Unless you're secretly Haskell programmers who snuck into a Rust class (in which case, you should be teaching this class), you probably want to use values without taking ownership of them. This is called *borrowing* in Rust. It's very similar to passing by reference in C++.

```rust
fn do_something(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello");
    do_something(&s);
    println!("{}", s);
}
```

Adding the ampersand indicates that you want to borrow the value, rather than take ownership of it. Note that you have to explicitly borrow at both the call site and in the function signature. This is a deliberate choice to avoid the "oh no, I just copied a huge data structure" problem that C++ has.

> Borrowing is a contract between the caller and the callee. The caller promises to keep the value alive for the duration of the borrow, and the callee promises not to modify or free the value (and the compiler enforces this).

## Mutable Borrowing

If you need to modify the borrowed value, you can create a mutable borrow. This is done by adding the `mut` keyword:

```rust
fn do_something(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hello");
    do_something(&mut s);
    println!("{}", s);
}
```

In this example, we create a mutable reference to `s` and pass it to `do_something`. This allows the function to modify the original string. Also note that:
* `s` must be declared as `mut` in order to be mutably borrowed.
* You must use `&mut` to create a mutable reference on the call-site.
* The function signature must also use `&mut` to indicate that it expects a mutable reference.

## Structs and Methods

So now the signature:

```rust
struct MyStruct {
    s: String,
}

impl MyStruct {
    fn do_something(&self) {
        println!("{}", self.s);
    }

    fn do_something_mut(&mut self) {
        self.s.push_str(", world!");
    }
}

fn main() {
    let mut my_struct = MyStruct {
        s: String::from("Hello"),
    };
    my_struct.do_something();
    my_struct.do_something_mut();
}
```

Since methods are just functions with a special first argument, the same rules apply. `&self` is a shorthand for `self: &Self`, and `&mut self` is a shorthand for `self: &mut Self`. This is why you see `&self` and `&mut self` in method signatures.

You *do* sometimes see `mut self` or `self` in method signatures. The method takes ownership of the struct instance, which means it can modify it without needing a mutable reference. This is common in builder patterns, where you want to consume the instance and return a new one.

## Dangling References

If you've ever used C++ (or C with pointers - since C doesn't have references), it's pretty likely that at *some* point you've created a dangling reference. This is a reference to a value that has been freed or moved. Rust prevents this at compile time. We'll talk about lifetimes later.

So this won't compile:

```rust
fn main() {
    let s = String::from("Hello");
    let r = &s;
    drop(s);
    println!("{}", r);
}
```

Notice that the compiler helpfully suggests using `clone()`. Cloning takes a deep copy of the variable, which can be a performance problem if the variable is large. It's also *really* common to clone more than you need to while learning the language.

> A side-effect of this is that it's not very common in Rust to take references and keep them around. You *can*, and the lifetimes system allows you to do so safely (although self-references are *gnarly*) - but most of the time Rust programmers avoid this.

