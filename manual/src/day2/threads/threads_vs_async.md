# Threads vs Async Overview

Rust has *two* concurrency models build in: threaded and async. The two can even mix!

**Threads** are pure "system threads" - just like `pthreads` in C or `std::thread` in C++. They are a way to run multiple tasks concurrently, sharing memory and resources. When a thread is created, it gains an entity in the operating system scheduler. Switching threads is a relatively (by kernel standards) expensive operation; the kernel restores the thread's state, registers, and stack. The thread runs, and then the kernel saves its state and switches to another thread. This is called "context switching".

**Async** is a way to run multiple *tasks* concurrently, often on a single thread. We'll cover it tomorrow. Think of async as cooperative multitasking.
