# Algebraic Data Types

A Rust enumeration is really a `tagged union` in C, or an `std::variant` in C++. This is a type that can be one of several different types, but only one at a time. Its size is the size of its largest variant, plus a tag to indicate which variant it is.

Rust enumerations are also called `Algebraic Data Types` (ADTs) because they can be combined using `sum` and `product` types. A `sum` type is an enum, where a value can be one of several different types. A `product` type is a struct, where a value contains several different types.

> Rust enumerations are *incredibly powerful*. A Rust idiom is to try and make invalid states unrepresentable. Enums are usually how this is implemented.

Let's define an enumeration:

```rust
enum Shape {
    Circle(f64),          // Enumerations can hold nameless values, like a tuple struct
    Rectange(width: f64, height: f64), // Or named values, like a regular struct\
    Point, // or no value at all
}
```

You can define methods on enumerations:

```rust
enum Shape {
    Circle(f64),          // Enumerations can hold nameless values, like a tuple struct
    Rectange(width: f64, height: f64), // Or named values, like a regular struct\
    Point, // or no value at all
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectange { width, height } => width * height,
            Shape::Point => 0.0,
        }
    }
}

fn main() {
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectange { width: 3.0, height: 4.0 };
    let point = Shape::Point;

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
    println!("Point area: {}", point.area());
}
```

You can also benefit from the matching system to destructure the enumeration:

```rust
enum LoginResult {
    Denied { reason: String },
    Accepted { user_id: u32 },
    SuperAdmin { user_id: u32 },
}

fn login(user: &str, password: &str) -> Result<Option<LoginResult>, String> {
    // If the database had crashed, we'd return Err("Database is down".to_string())
    if user == "admin" && password == "password" {
        Ok(Some(LoginResult::SuperAdmin { user_id: 1 }))
    } else if user == "user" && password == "password" {
        Ok(Some(LoginResult::Accepted { user_id: 2 }))
    } else if user == "user" {
        Ok(Some(LoginResult::Denied { reason: "Wrong password".to_string() }))
    } else {
        // There is no user. This is not an error, so we return None.
        Ok(None)
    }
}

fn main() {
    match login("admin", "password") {
        Ok(Some(LoginResult::SuperAdmin { user_id })) => println!("Welcome, super admin {}!", user_id),
        Ok(Some(LoginResult::Accepted { user_id })) => println!("Welcome, user {}!", user_id),
        Ok(Some(LoginResult::Denied { reason })) => println!("Access denied: {}", reason),
        Ok(None) => println!("No such user"),
        Err(e) => println!("Error: {}", e),
    }
}
```

> Don't accidentally let your IDE fill in every possible match arm for an integer. It happens to all of us at least once!