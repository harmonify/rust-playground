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
    let bank_balance = 1000.00;
    
    age = 31;
    println!("{name} is {age} years old and has ${bank_balance} in his bank account\n");
    
    print_primitive_types_size();
}

fn print_primitive_types_size() {
    println!("\nPrimitive types sizes:");

    println!("Integer 32 size: {:?}", std::mem::size_of::<i32>());
    println!("Integer 64 size: {:?}", std::mem::size_of::<i64>());
    println!("Unsigned integer 32 size: {:?}", std::mem::size_of::<u32>());

    println!("Float 64 size: {:?}", std::mem::size_of::<f64>());

    println!("boolean size: {:?}", std::mem::size_of::<bool>());
    println!("char size: {:?}", std::mem::size_of::<char>());
    println!("string size: {:?}", std::mem::size_of::<&str>());
}
