/// # Primitive Types
///
/// Variables hold primitive data or references to data
/// Variables are immutable by default
/// Rust is a block-scoped language
///
/// ## Integers
///
/// i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
///
/// By default, the integer type is signed (i32).
/// Note that u stands for unsigned integer.
///
/// ## Floating-point numbers
///
/// f32, f64
///
/// By default, the floating-point type is f64.
///
/// ## Boolean
///
/// bool
///
/// ## Character
///
/// char
///

pub fn run() {
    let name = "John"; // by default, immutable
    let mut age = 30; // mutable
    age = 31;
    let bank_balance = 1000.00;
    let emoji = '\u{1F600}';

    println!("{name} is {age} years old and has ${bank_balance} in his bank account {emoji}\n");

    print_primitive_types_size();
}

fn print_primitive_types_size() {
    println!("\nPrimitive types sizes:");

    println!("i32 min: {}; i32 max: {}; i32 size: {}", i32::MIN, i32::MAX, std::mem::size_of::<i32>());
    println!("i64 min: {}; i64 max: {}; i64 size: {}", i64::MIN, i64::MAX, std::mem::size_of::<i64>());
    println!("u32 min: {}; u32 max: {}; u32 size: {}", u32::MIN, u32::MAX, std::mem::size_of::<u32>());

    println!("f64 min: {}; f64 max: {}; f64 size: {}", f64::MIN, f64::MAX, std::mem::size_of::<f64>());

    println!("bool size: {:?}", std::mem::size_of::<bool>());
    println!("char size: {:?}", std::mem::size_of::<char>());
    println!("&str size: {:?}", std::mem::size_of::<&str>());
    println!();
}
