use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Write},
    fs,
};

fn main() {
    let filename = "inputs/14.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let platform = content
        .lines()
        .map(|l| {
            l.chars()
                .map(Tile::try_from)
                .collect::<Result<Vec<Tile>, ParseTileError>>()
                .expect("Should have been able to parse tiles")
        })
        .collect::<Vec<Vec<Tile>>>();

    let mut p = platform.clone();
    tilt_north(&mut p);
    let load = p
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&&t| t == Tile::RoundRock).count() * (platform.len() - i)
        })
        .sum::<usize>();
    println!("The total load on the north support beams is {load}.");

    let n_cycles = 1_000_000_000;
    let mut p = platform.clone();
    let mut visited = HashMap::new();
    'outer: for i in 0..n_cycles {
        let s = to_string(&p);
        if visited.contains_key(&s) {
            // let's fast forward
            let period = i - visited[&s];
            // period doesn't cancel out due to integer/floor division!
            let i = i + (n_cycles - i) / period * period;
            for _ in i..n_cycles {
                cycle(&mut p);
            }
            break 'outer;
        }
        cycle(&mut p);
        visited.insert(s, i);
    }
    let load = p
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&&t| t == Tile::RoundRock).count() * (platform.len() - i)
        })
        .sum::<usize>();
    println!("The total load on the north support beams after {n_cycles} cycles is {load}.");
}

fn cycle(platform: &mut [Vec<Tile>]) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn tilt_north(platform: &mut [Vec<Tile>]) {
    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            for offset in 1..(i + 1) {
                let next_idx = i - offset;
                let current_idx = i - offset + 1;
                if platform[current_idx][j] == Tile::RoundRock
                    && platform[next_idx][j] == Tile::Floor
                {
                    platform[current_idx][j] = Tile::Floor;
                    platform[next_idx][j] = Tile::RoundRock;
                }
            }
        }
    }
}

fn tilt_south(platform: &mut [Vec<Tile>]) {
    for i in (0..platform.len()).rev() {
        for j in 0..platform[i].len() {
            for offset in 1..(platform.len() - i) {
                let next_idx = i + offset;
                let current_idx = i + offset - 1;
                if platform[current_idx][j] == Tile::RoundRock
                    && platform[next_idx][j] == Tile::Floor
                {
                    platform[current_idx][j] = Tile::Floor;
                    platform[next_idx][j] = Tile::RoundRock;
                }
            }
        }
    }
}

fn tilt_west(platform: &mut [Vec<Tile>]) {
    for row in platform.iter_mut() {
        for j in 0..row.len() {
            for offset in 1..(j + 1) {
                let next_idx = j - offset;
                let current_idx = j - offset + 1;
                if row[current_idx] == Tile::RoundRock && row[next_idx] == Tile::Floor {
                    row[current_idx] = Tile::Floor;
                    row[next_idx] = Tile::RoundRock;
                }
            }
        }
    }
}

fn tilt_east(platform: &mut [Vec<Tile>]) {
    for row in platform.iter_mut() {
        for j in (0..row.len()).rev() {
            for offset in 1..(row.len() - j) {
                let next_idx = j + offset;
                let current_idx = j + offset - 1;
                if row[current_idx] == Tile::RoundRock && row[next_idx] == Tile::Floor {
                    row[current_idx] = Tile::Floor;
                    row[next_idx] = Tile::RoundRock;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Floor,
    RoundRock,
    CubeRock,
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Floor),
            'O' => Ok(Tile::RoundRock),
            '#' => Ok(Tile::CubeRock),
            _ => Err(ParseTileError),
        }
    }
}

#[derive(Debug)]
struct ParseTileError;

impl fmt::Display for ParseTileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseTileError {}

fn to_string(platform: &[Vec<Tile>]) -> String {
    platform
        .iter()
        .map(|l| {
            let mut s = l.iter().fold(String::new(), |mut output, c| {
                let _ = write!(output, "{c}");
                output
            });
            s.push('\n');
            s
        })
        .collect::<String>()
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Floor => '.',
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
        };
        write!(f, "{}", c)
    }
}
