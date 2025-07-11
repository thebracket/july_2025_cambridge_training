# Cargo Extensions

Just in case you didn't think `Cargo` was complete, you can use `cargo install` to add plugins to it. I frequently use `cargo audit` to alert me to any reported security vulnerabilities in my codebase, and `cargo deny` is useful for catching things like "oh no, I tried to link a GPL'd project - and now legal are angry".

Other useful extensions:

* `cargo watch` — Rebuild or rerun tests every time a file changes.
* `cargo nextest` — A faster test runner for big test suites.
