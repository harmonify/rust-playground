pub fn run() {
    // static array
    // let static_array: [i32; 5] = [4, 2, 6, 8, 10];
    // let sliced_array = &static_array[1..3]; // [2, 6]
    // println!("{:?}", sliced_array);

    // dynamic array
    // let mut vector: Vec<String> = vec![];
    // for (i, value) in static_array.iter().enumerate() {
    //     println!("static_array[{}]: {}", i, value);
    //     vector.push(format!("Hello {}", value));
    // }
    // println!("{:?}", vector);

    // more array methods
    let sample = [1, 2, 3, 4, 5];
    let mut example1 = vec![]; // [1, 2, 3, 4, 5]
    for x in sample.iter() {
        example1.push(x * x);
    }
    println!("{:?}", example1);

    // `map()` is lazy evaluated, `for` loop is eager evaluated
    let example2 = sample.iter().map(|x| x * 2).collect::<Vec<i32>>(); // [4, 12]
    println!("{:?}", example2);
}
