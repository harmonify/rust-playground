enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Player {
    name: String,
    gender: String,
    position: Position,
}

impl Player {
    fn new(name: &str, gender: &str) -> Player {
        Player {
            name: name.to_string(),
            gender: gender.to_string(),
            position: Position { x: 0, y: 0 },
        }
    }

    /// Move avatar based on the `direction` and `distance` parameter
    /// and return the reference to the `self.position`.
    fn move_avatar(&mut self, direction: Direction, distance: i32) -> Position {
        match direction {
            Direction::North => {
                println!("{} is moving to the North by {}px", self.name, distance);
                self.position.y += distance;
                self.position
            }
            Direction::East => {
                println!("{} is moving to the East by {}px", self.name, distance);
                self.position.x += distance;
                self.position
            }
            Direction::South => {
                println!("{} is moving to the South by {}px", self.name, distance);
                self.position.y -= distance;
                self.position
            }
            Direction::West => {
                println!("{} is moving to the West by {}px", self.name, distance);
                self.position.x -= distance;
                self.position
            }
        }
    }

    fn coordinates(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }
}

pub fn run() {
    let mut bob = Player::new("Bob", "Prefer not to say");
    println!("{} has joined the lobby", bob.name);
    println!("{} coordinates: {:?}", bob.name, bob.coordinates());
    bob.move_avatar(Direction::East, 70);
    bob.move_avatar(Direction::North, 40);
    bob.move_avatar(Direction::South, 200);
    println!("{} coordinates: {:?}", bob.name, bob.coordinates());
}
