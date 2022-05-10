pub fn run() {
    // function currying
    let pow_by_5 = math_power_by(5);
    println!("2^5 is {}", pow_by_5(2));
    println!("1^5 is {}", pow_by_5(1));
    println!("3^5 is {}", pow_by_5(3));
}

fn math_power_by(exponent: u32) -> impl Fn(i32) -> i32 {
    return move |base: i32| base.pow(exponent);
}
