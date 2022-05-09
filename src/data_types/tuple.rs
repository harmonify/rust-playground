pub fn tuple() {
    let wendy: (&str, i32) = ("Wendy", 20);
    println!("{} is {} years old", wendy.0, wendy.1);
}
