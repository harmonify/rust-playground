/// There are many common built-in types in Rust.

pub fn run() {
    self::option();
}

/// Option<T> is a type that can be either Some<T> or None
/// kinda like a nullable variable in javascript
/// use .unwrap_or("default") to get the value or a default value ("default" in this case)
fn option() {
    // example greeting
    let mut name: Option<&str> = None;
    name = Some("John");
    println!("Hello, {}", name.unwrap_or("World"));
}
