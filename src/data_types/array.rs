pub fn array() {
    // static array
    let static_array: [i32; 5] = [4, 2, 6, 8, 10];

    // dynamic array
    let mut vector: Vec<String> = vec![];

    for (i, value) in static_array.iter().enumerate() {
        println!("static_array[{}]: {}", i, value);

        vector.push(format!("Hello {}", value));
    }

    println!("{:?}", vector);
}
