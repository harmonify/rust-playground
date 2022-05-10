/// The Ownership and Borrowing concept in Rust
/// provides both memory safety and low level memory management.

pub fn owned_pointer() {
    // Owned pointer – only one thing can ‘own’ this pointer at a time
    // This allows variables to be automatically deallocated safely.
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // dereference

    // Here, `now_its_mine` takes ownership of `mine`. In other words, `mine` is moved.
    let mut now_its_mine = mine;
    *now_its_mine += 2;

    println!("Owned pointer (now_its_mine) value: {}\n", now_its_mine); // 7
    // println!("{}", mine); // this would not compile because `now_its_mine` now owns the pointer
}

pub fn reference() {
    // Reference – an immutable pointer that refers to other data
    // When a reference is taken to a value, we say that the value has been `borrowed`.
    // While a value is borrowed immutably, it cannot be `mutated` or `moved`.
    // A borrow is active until the last use of the borrowing variable.
    let mut owner = String::from("John");
    owner = String::from("Jane");
    let borrower = &owner;
    // owner = String::from("Jack"); // error: cannot assign to `owner` because it is borrowed
    println!("Borrower: {}", borrower); // `owner` is freed after this line
    owner = String::from("Jack"); // okay
    println!("Owner after reassignment: {}\n", owner); // `owner` is freed after this line
}

pub fn mutable_reference() {
    // Mutable reference
    // While a value is mutably borrowed, it cannot be accessed at all.
    let mut people_age: Vec<i32> = vec![18, 25, 23, 28, 30];
    println!("People age now: {:?}", people_age);

    // borrow the vector
    let people_age_borrower = &mut people_age;
    // one year passed so...
    add_one_to_all_vector_elements(people_age_borrower);

    // the vector is no longer borrowed and the next line
    // will print just fine
    println!("People age 1 year later: {:?}\n", people_age);
}

pub fn add_one_to_all_vector_elements(vec: &mut Vec<i32>) {
    for i in vec {
        *i += 1;
    }
}
