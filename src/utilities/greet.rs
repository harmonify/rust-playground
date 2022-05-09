// Option<T> is a type that can be either Some<T> or None
// kinda like a nullable variable in javascript
// use .unwrap_or("default") to get the value or a default value ("default" in this case)

pub fn run(name: Option<&str>) {
    let n: &str = name.unwrap_or("World");
    println!("Hello, {}", n);
}
