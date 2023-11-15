use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_input(str: &str) -> (bool, i32) {
    let distance: i32 = str[1..].trim().parse().expect("Failed to parse number");
    if str.strip_prefix('R').is_some() {
        (true, distance)
    } else {
        (false, distance)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut dir = Direction::North;
    let mut result = (0,0);

    let instructions: Vec<(bool, i32)> = input.split(", ").map(parse_input).collect();
    for (RorL, dist) in instructions {
        if RorL {
            dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
            };
        } else {
            dir = match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
            };
        }
        match dir {
            Direction::North => x += dist,
            Direction::East => y += dist,
            Direction::South => x -= dist,
            Direction::West => y -= dist,
        }
        if set.contains(&(x,y)) {
            result = (x,y);
            break;
        } else {
            set.insert((x,y));
        }
    }

    println!("{}", result.0.abs() + result.1.abs());

}
