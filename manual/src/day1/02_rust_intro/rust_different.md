# How is Rust Different?

I understand a fair number of you come from Python, Java, C and C++ backgrounds. That’s great — Rust borrows (pun intended) a lot of ideas from those ecosystems.

How many of you have written Rust beyond Hello World?

(Show of Hands)

How many of you have some Rust in production?

(Show of Hands)

## Rust is a Compiled Language Without a Garbage Collector

|Language|Target|Memory|
|--------|------|------|
|Python|Interpreted at Runtime|Manged/Garbage Collected|
|Java and C#|Compiles to Java Virtual Machine Bytecode|Manged/Garbage Collected|
|Go|Compiles to Semi-Optimized Native Code with an Opinioned Runtime|Manged/Garbage Collected|
|C and C++|Compiles to Optimied Native Code|Manual/Smart Pointers|
|Rust|Compiles to Optimized Native Code, Static Linking by Default|Ownership/Smart Pointers|

Rust manages memory via **ownership** and **borrowing**, which gives you performance and safety — *without* needing a garbage collector. But unless you're doing very low-level work, you often don’t need to think about memory management explicitly. The compiler helps a lot.

## Rust is a General Purpose Systems Language

Rust can be used for the really low-level stuff: you can write an operating system, or an embedded platform. Rust can *also* scale all the way up to high-level web constructs.

## Rewrite Everything in Rust!!!

Let’s get this out of the way early: I’m *not* one of those people who reply to every tech article with “should’ve written it in Rust.”

There's lots of room for different languages; choose the right tool for the job!
