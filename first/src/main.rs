#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    
    let input = include_str!("./input.txt");
    let (_, x, y): (Direction, i32, i32) = input.split(", ")
        .fold((Direction::North, 0, 0), |(mut dir, x, y), str| {
            let distance: i32 = if let Some(num) = str.strip_prefix('R') {
                dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
                num.trim().parse().expect(&format!("Failed to parse distance from string {num}"))
            } else if let Some(num) = str.strip_prefix('L') {
                dir = match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                };
                num.trim().parse().expect(&format!("Failed to parse distance from string {num}"))
            } else {
                panic!("Invalid Direction")
            };
            match dir {
                Direction::North => (dir, x + distance, y),
                Direction::East => (dir, x, y + distance),
                Direction::South => (dir, x - distance, y),
                Direction::West => (dir, x, y - distance),
            }
    });

    println!("Total distance away is {}", x.abs() + y.abs());

}
