# WS: Loading your Rooms with proper error handling

This shouldn't take too long. Go back to your `rooms` and `users` projects from yesterday. Replace all of your `Result<.., String>` with your own `ThisError` type --- and update the client code to use `anyhow`.

You can find my versions in the `code/day2` directory. I had to add a  "2" after the project names to avoid conflicts with the previous versions.

Note that I'm quite fond of using the `anyhow` crate for error handling in the main application logic, while keeping the `thiserror` crate for library-level errors. I also love the `anyhow::bail!` macro for quickly returning errors in the main function.