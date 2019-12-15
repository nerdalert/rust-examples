/// Enums allow you to define a type by enumerating its possible values
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let map_direction: Direction = Direction::East;
    // match the enum value we set above
    match map_direction {
        Direction::North => println!("North we go"),
        Direction::South => println!("South we go"),
        Direction::East => println!("East we go"),
        Direction::West => println!("West we go"),
    }
}

/*
The output is:
-------------
East we go
*/

