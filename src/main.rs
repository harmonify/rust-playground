// Notes:
// Note that the following Rust linter rules are unsafe to use at real projects
// The use of those rules is for the ease of demonstration only.
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

mod data_types;
mod concepts;

fn main() {
    println!("Hello, world!");

    // data_types::primitive_types::primitive_types();
    // data_types::arrays::arrays();
    // data_types::tuple::tuple();

    concepts::ownership_and_borrowing::ownership_and_borrowing();
}
