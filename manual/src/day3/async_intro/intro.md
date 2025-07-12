# Concurrency 2: Async

Threaded programming is great, but it has some downsides. Threads are relatively heavy weight, and context switching between them can be expensive. If you have a lot of tasks that spend most of their time waiting for I/O (like servers), threads can be inefficient.

Asynchronous programming is a different approach to concurrency---and can even run on a single thread. Instead of blocking while waiting for I/O, async tasks yield control back to a runtime, which can then run other tasks while waiting for I/O to complete. This can lead to more efficient use of resources, especially in I/O-bound applications.

nodejs, Python's asyncio, and C#'s async/await are all examples of async programming in other languages.

## Cooperative Multitasking

A key concept in async programming is cooperative multitasking. If you remember back to pre version X Mac OS, Windows 3.x, or even computers like the Amiga, they used cooperative multitasking. Each program had to yield control back to the OS periodically, or the whole system would freeze.

Async programming is similar---each async task has to yield control back to the runtime when it is waiting for another task (or I/O) to complete. Hogging the CPU in an async task will block all other tasks. Sleeping the current thread can be disastrous in async code, as it blocks the entire runtime!