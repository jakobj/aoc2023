use std::fs;

fn main() {
    let filename = "inputs/18.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let instructions = content
        .lines()
        .map(|l| {
            let l = l.split(' ').collect::<Vec<&str>>();
            (
                Instruction {
                    direction: Direction::from(l[0]),
                    distance: l[1].parse::<i32>().unwrap(),
                },
                Color::from(&l[2][2..l[2].len() - 1]),
            )
        })
        .collect::<Vec<(Instruction, Color)>>();

    // dig loop
    let perimeter = dig_perimeter(&instructions, false);
    let volume = compute_volume(&perimeter);
    println!("The lagoon could hold {volume} cubic meters of lava.");

    // dig loop with decoded colors
    let perimeter = dig_perimeter(&instructions, true);
    let volume = compute_volume(&perimeter);
    println!("The lagoon could hold {volume} cubic meters of lava.");
}

fn dig_perimeter(instructions: &[(Instruction, Color)], use_color: bool) -> Vec<Position> {
    let mut perimeter = Vec::new();
    let mut position = Position { y: 0, x: 0 };
    perimeter.push(position);
    for (mut ins, color) in instructions.iter() {
        if use_color {
            ins = color.decode();
        }
        for _ in 0..ins.distance {
            (position.y, position.x) = match ins.direction {
                Direction::Up => (position.y - 1, position.x),
                Direction::Down => (position.y + 1, position.x),
                Direction::Left => (position.y, position.x - 1),
                Direction::Right => (position.y, position.x + 1),
            };
            perimeter.push(position);
        }
    }
    assert!(position.y == 0 && position.x == 0);
    perimeter.pop();
    perimeter
}

fn compute_volume(perimeter: &[Position]) -> i64 {
    // calculate are of polygon
    // https://stackoverflow.com/questions/451426/how-do-i-calculate-the-area-of-a-2d-polygon
    // https://web.archive.org/web/20100405070507/http://valis.cs.uiuc.edu/~sariel/research/CG/compgeom/msg00831.html
    let mut volume: i64 = 0;
    for i in 0..perimeter.len() {
        let j = (i + 1) % perimeter.len();
        volume += (perimeter[i].x + 1) as i64 * (perimeter[j].y + 1) as i64;
        volume -= (perimeter[i].y + 1) as i64 * (perimeter[j].x + 1) as i64;
    }
    volume /= 2;

    // control for ring "area"
    volume += perimeter.len() as i64 / 2 + 1;
    volume
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
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

impl Color {
    fn decode(&self) -> Instruction {
        let s = format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b);
        let distance = i32::from_str_radix(&s[..s.len() - 1], 16).unwrap();
        let direction = match &s[s.len() - 1..s.len()] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Unexpected direction {}", &s[s.len() - 1..s.len()]),
        };
        Instruction {
            direction,
            distance,
        }
    }
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
