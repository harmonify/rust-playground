// Notes:
// Note that the following Rust linter rules are unsafe to use at real projects
// The use of those rules is for the ease of demonstration only.
// #[<rules>] for a specific module, function, etc.
// #![<rules>] for the whole project
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]

mod basic;
mod concepts;
mod utilities;

fn main() {
    // println!("Hello, world!\n");

    // basic::printings::run();

    // basic::primitive_types::run();
    // basic::arrays::run();
    // basic::tuples::run();
    // basic::common_types::run();

    // basic::control_flows::run();

    basic::functions::run();
    
    // basic::user_inputs::greet_name();
    // basic::user_inputs::input_abc();

    // utilities::greet::run(None);
    // utilities::greet::run(Some("John"));

    // concepts::ownership_and_borrowing::owned_pointer();
    // concepts::ownership_and_borrowing::reference();
    // concepts::ownership_and_borrowing::mutable_reference();
}
