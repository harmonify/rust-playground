struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn new(first_name: &str, last_name: &str, age: u32) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn is_of_age(&self) -> bool {
        self.age >= 21
    }
}

pub fn run() {
    let mut bob = Person::new("John", "Bob", 22);
    bob.first_name = "Bobby".to_string();
    println!("{} is {} years old", bob.full_name(), bob.age);
}
