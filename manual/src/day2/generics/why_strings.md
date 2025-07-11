# Aside: Why so many strings?

I recently proof-read a book proposal that started with `String` as the first type. I marked it as a "brave choice", because Rust has *so many* string types---and their behavior can be a bit confusing.

String types can include:

* `&str` - a slice of characters, often used for string literals. You can take a string literal and turn it into a `&str` by borrowing it, like `let s: &str = "Hello";`.
* `String` - an owned string type, which is mutable and can grow. You can create it with `let s = String::from("Hello");`.
* `Cow<str>` - a "clone on write" type that can be either a `&str` or a `String`. It's useful when you want to avoid unnecessary cloning. You can create it with `let s: Cow::Borrowed("Hello");` or `let s: Cow<str> = Cow::Owned(String::from("Hello"));`.
* `OsString` - a string type for operating system strings, which can handle non-UTF-8 data. You can create it with `let s = OsString::from("Hello");`.
* `CString` - a string type for C-compatible strings, which ensures null-termination. You can create it with `let s = CString::new("Hello").unwrap();`.
* `cstr` - a slice of C-compatible strings, which is often used in FFI (Foreign Function Interface) scenarios. You can create it with `let s: &CStr = CString::new("Hello").unwrap().as_c_str();`.
* `Path` and `PathBuf` - while not strictly string types, they are often used to represent file paths and can be converted to strings. You can create a `PathBuf` with `let p = PathBuf::from("path/to/file");`.

There are more - plenty more when you add crates!

Just to further confuse you, how big is a `char`? It's 4 bytes, but it can represent a Unicode character, which can be more than one byte in UTF-8. So, a `String` can contain many `char`s, and each `char` can take up to 4 bytes.