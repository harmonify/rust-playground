// Notes:
// Note that the following Rust linter rules are unsafe to use at real projects
// The use of those rules is for the ease of demonstration only.
// #[<rules>] for a specific module, function, etc.
// #![<rules>] for the whole project
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

mod basic;
mod variables;
mod concepts;
mod utilities;

fn main() {
    // println!("Hello, world!\n");

    // basic::printing::run();

    // utilities::greet::run(None);
    // utilities::greet::run(Some("John"));    

    variables::primitive_types::run();
    // variables::arrays::run();
    // variables::tuple::run();

    // concepts::ownership_and_borrowing::owned_pointer();
    // concepts::ownership_and_borrowing::reference();
    // concepts::ownership_and_borrowing::mutable_reference();
}
