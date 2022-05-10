use std::io::{stdout, Write};
use text_io::{read, scan};
// tldr: `read` for 1 input, `scan` for multiple inputs

pub fn greet_name() {
    print!("Enter your name: ");
    stdout().flush().unwrap();
    // read macro from text_io requires heap allocated (dynamic sized) string
    let name: String = read!();
    println!("Hello, {}!", name);
}

pub fn input_abc() {
    // reading multiple values from stdio
    let a: i32;
    let b: &mut f32 = &mut 0.0;
    let c: String;

    // scan!("{}, {}", a, *b);
    print!("Enter integer a: ");
    stdout().flush().unwrap();
    scan!("{}", a);

    print!("Enter floating number b: ");
    stdout().flush().unwrap();
    scan!("{}", *b);

    print!("Enter string c: ");
    stdout().flush().unwrap();
    scan!("{}", c);

    // sleep for 2 seconds
    std::thread::sleep(std::time::Duration::from_secs(2));

    // print out a: type = value
    println!("a: i32 = {}, b: &mut f32 = {}, c: &str = {}", a, b, c);
}
