use std::{error::Error, fmt, fs};

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
    let platform = tilt(&platform);
    let load = platform
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&&t| t == Tile::RoundRock).count() * (platform.len() - i)
        })
        .sum::<usize>();
    println!("The total load on the north support beams is {load}.");
}

fn tilt(platform: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut platform = platform.to_vec();
    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            for offset in 1..(i + 1) {
                if platform[i - offset + 1][j] == Tile::RoundRock
                    && platform[i - offset][j] == Tile::Floor
                {
                    platform[i - offset + 1][j] = Tile::Floor;
                    platform[i - offset][j] = Tile::RoundRock;
                }
            }
        }
    }
    platform
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
