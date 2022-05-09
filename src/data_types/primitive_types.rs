pub fn primitive_types() {
    println!("\nPrimitive types:");
    // print all the primitive types in the Rust language
    println!("Integer 8 size: {:?}", std::mem::size_of::<i8>());
    println!("Integer 16 size: {:?}", std::mem::size_of::<i16>());
    println!("Integer 32 size: {:?}", std::mem::size_of::<i32>());
    println!("Integer 64 size: {:?}", std::mem::size_of::<i64>());

    println!("Float 32 size: {:?}", std::mem::size_of::<f32>());
    println!("Float 64 size: {:?}", std::mem::size_of::<f64>());

    println!("Unsigned integer 8 size: {:?}", std::mem::size_of::<u8>());
    println!("Unsigned integer 16 size: {:?}", std::mem::size_of::<u16>());
    println!("Unsigned integer 32 size: {:?}", std::mem::size_of::<u32>());
    println!("Unsigned integer 64 size: {:?}", std::mem::size_of::<u64>());

    println!("boolean size: {:?}", std::mem::size_of::<bool>());
    println!("char size: {:?}", std::mem::size_of::<char>());
    println!("string size: {:?}", std::mem::size_of::<&str>());

    println!("usize size: {:?}", std::mem::size_of::<usize>());
    println!("isize size: {:?}", std::mem::size_of::<isize>());
}
