# Raw Pointers DO Exist

You usually see them in systems-level code or FFI (Foreign Function Interface) situations, but they are part of Rust's capabilities. Raw pointers are not commonly used in everyday Rust programming because Rust provides safer abstractions like smart pointers (`Box`, `Rc`, `Arc`) that manage memory automatically.

## Example of Raw Pointers

```rust
fn main() {
    let x = 42;
    let r: *const i32 = &x;

    unsafe {
        println!("r points to: {}", *r);
    }
}
```

Unlike C, Rust pointers are either `*const T` (immutable) or `*mut T` (mutable). You **must** use `unsafe` blocks to dereference raw pointers, as Rust cannot guarantee their safety.

Just in case any of you are really pining for C, yes there is a sneaky little `void` pointer in Rust:

```rust
use std::ffi::c_void;

fn main() {
    let x = 42;
    let r = &x as *const i32 as *const c_void;
    unsafe {
        println!("r points to: {}", *(r as *const i32));
    }
}
```

All of the pointer math is still there, too!

```rust
fn main() {
    let x = [42, 43, 44];
    let r: *const i32 = &x[0];
    unsafe {
        println!("First element: {}", *r);
        println!("Second element: {}", *(r.add(1))); // Pointer arithmetic
        println!("Third element: {}", *(r.add(2)));
        // r.add(3) would be out of bounds and cause undefined behavior. We're in unsafe land!
    }
}

