enum Direction {
    North,
    East,
    South,
    West,
}

pub fn run() {
    // match is identical to switch statement in another programming languages
    let bob_destination = Direction::North;
    match bob_destination {
        Direction::North => println!("Bob is heading towards North"),
        Direction::East => println!("Bob is heading towards East"),
        Direction::South => println!("Bob is heading towards South"),
        Direction::West => println!("Bob is heading towards West"),
    }
}
