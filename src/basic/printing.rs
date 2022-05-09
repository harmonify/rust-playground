#[allow(clippy::print_literal)]

pub fn run() {
    // Print to console
    println!("Hello from print.rs!");

    // Print to console with a format string
    println!("I'm a {}! {} points for Rust!", "Rustacean 🦀", 100);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Pico", "Japan", "code"
    );

    // Named Arguments
    println!(
        "{name} is from {country} and {name} likes to {activity}",
        name = "Pico",
        country = "Japan",
        activity = "code"
    );

    // Placeholder
    let brad = ("Pico", "Japan", "code");
    let (name, country, activity) = brad;
    println!("{name} is from {country} and {name} likes to {activity}");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}\n", (12, true, "Hello!")); 
}
