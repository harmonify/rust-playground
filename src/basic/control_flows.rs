#[allow(clippy::println_empty_string)]

pub fn run() {
    // `if` statement
    let age: u8 = 20;
    if age >= 21 {
        println!("You can drink");
    } else if age == 20 {
        println!("Sorry buddy wait for another year");
    } else {
        println!("You can't drink");
    }

    // `if` as expression (ternary)
    let teeth_is_healthy: bool = true;
    let can_eat_chocolate: &str = if teeth_is_healthy {
        "Yes, they can"
    } else {
        "No, they CANNOT"
    };

    // Ranges
    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }

    for i in 0u32..10 {
        print!("{i} "); // prints `0 1 2 3 4 5 6 7 8 9 `
    }
    println!("");
}
