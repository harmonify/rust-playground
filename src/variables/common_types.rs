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

/// Primitive str = Immutable fixed-length string somewhere in memory
/// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
fn string() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
  
    // Push char
    hello.push('W');
  
    // Push string
    hello.push_str("orld!");
  
    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
  
    // Check if empty
    println!("Is Empty: {}", hello.is_empty());
  
    // Contains
    println!("Contains 'World' {}", hello.contains("World"));
  
    // Replace
    println!("Replace: {}", hello.replace("World", "There"));
  
    // Loop through string by whitespace
    for word in hello.split_whitespace() {
      println!("{}", word);
    }
  
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
  
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
  
    println!("{}", s);
}
