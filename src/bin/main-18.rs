use std::fs;

fn main() {
    let filename = "inputs/18.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let instructions = content
        .lines()
        .map(|l| {
            let l = l.split(' ').collect::<Vec<&str>>();
            Instruction {
                direction: Direction::from(l[0]),
                distance: l[1].parse::<i32>().unwrap(),
                color: Color::from(&l[2][2..l[2].len() - 1]),
            }
        })
        .collect::<Vec<Instruction>>();

    // dig loop
    let mut ring = Vec::new();
    let mut position = Position { y: 0, x: 0 };
    ring.push(position);
    for ins in instructions.iter() {
        for _ in 0..ins.distance {
            (position.y, position.x) = match ins.direction {
                Direction::Up => (position.y - 1, position.x),
                Direction::Down => (position.y + 1, position.x),
                Direction::Left => (position.y, position.x - 1),
                Direction::Right => (position.y, position.x + 1),
            };
            ring.push(position);
        }
    }
    assert!(position.y == 0 && position.x == 0);
    ring.pop();

    // calculate are of polygon
    // https://stackoverflow.com/questions/451426/how-do-i-calculate-the-area-of-a-2d-polygon
    // https://web.archive.org/web/20100405070507/http://valis.cs.uiuc.edu/~sariel/research/CG/compgeom/msg00831.html
    let mut volume = 0;
    for i in 0..ring.len() {
        let j = (i + 1) % ring.len();
        volume += (ring[i].x + 1) * (ring[j].y + 1);
        volume -= (ring[i].y + 1) * (ring[j].x + 1);
    }
    volume /= 2;

    // control for ring "area"
    volume += ring.len() as i32 / 2 + 1;

    println!("The lagoon could hold {volume} cubic meters of lava.")
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
    color: Color,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unexpected direction {s}"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Color {
            r: u8::from_str_radix(&s[..2], 16).unwrap(),
            g: u8::from_str_radix(&s[2..4], 16).unwrap(),
            b: u8::from_str_radix(&s[4..], 16).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Position {
    y: i32,
    x: i32,
}
