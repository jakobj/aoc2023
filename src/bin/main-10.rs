use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/10.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let map = parse_sketch(&content).expect("Should have been able to parse sketch");

    // bfs from animal position without repeated visits
    let animal_position = find_animal(&map).expect("Could not find animal position");
    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(State {
        steps: 0,
        position: animal_position,
    });
    let mut max_steps = 0;
    while let Some(current_state) = stack.pop() {
        if visited.contains(&current_state.position) {
            continue;
        }
        visited.insert(current_state.position);
        max_steps = std::cmp::max(max_steps, current_state.steps);
        let neighbors = get_neighbors(&map, current_state.position);
        for n in neighbors {
            stack.push(State {
                steps: current_state.steps + 1,
                position: n,
            });
        }
    }
    println!("It takes {max_steps} steps along the loop to get from the starting position to the point farthest from the starting position.");
}

fn parse_sketch(content: &str) -> Result<Vec<Vec<Tile>>, TileParseError> {
    content
        .lines()
        .map(|l| {
            l.chars()
                .map(Tile::try_from)
                .collect::<Result<Vec<Tile>, TileParseError>>()
        })
        .collect::<Result<Vec<Vec<Tile>>, TileParseError>>()
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Animal,
}

impl TryFrom<char> for Tile {
    type Error = TileParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Tile::Vertical),
            '-' => Ok(Tile::Horizontal),
            'L' => Ok(Tile::NE),
            'J' => Ok(Tile::NW),
            '7' => Ok(Tile::SW),
            'F' => Ok(Tile::SE),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Animal),
            _ => Err(TileParseError),
        }
    }
}

#[derive(Debug)]
struct TileParseError;

fn find_animal(map: &[Vec<Tile>]) -> Result<Position, AnimalNotFoundError> {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::Animal {
                return Ok(Position { y: i, x: j });
            }
        }
    }
    Err(AnimalNotFoundError)
}

#[derive(Debug)]
struct AnimalNotFoundError;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Eq)]
struct State {
    steps: usize,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // turn it into a min-heap
        // (https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

fn get_neighbors(map: &Vec<Vec<Tile>>, position: Position) -> Vec<Position> {
    let connected_to_up = HashSet::from([Tile::Vertical, Tile::NW, Tile::NE]);
    let connected_to_down = HashSet::from([Tile::Vertical, Tile::SW, Tile::SE]);
    let connected_to_left = HashSet::from([Tile::Horizontal, Tile::NW, Tile::SW]);
    let connected_to_right = HashSet::from([Tile::Horizontal, Tile::NE, Tile::SE]);
    let mut neighbors = Vec::new();

    let current_tile = &map[position.y][position.x];
    if position.y > 0 {
        let next_tile = &map[position.y - 1][position.x];
        if (*current_tile == Tile::Animal || connected_to_up.contains(current_tile))
            && connected_to_down.contains(next_tile)
        {
            neighbors.push(Position {
                y: position.y - 1,
                x: position.x,
            });
        }
    }
    if position.y + 1 < map.len() {
        let next_tile = &map[position.y + 1][position.x];
        if (*current_tile == Tile::Animal || connected_to_down.contains(current_tile))
            && connected_to_up.contains(next_tile)
        {
            neighbors.push(Position {
                y: position.y + 1,
                x: position.x,
            });
        }
    }
    if position.x > 0 {
        let next_tile = &map[position.y][position.x - 1];
        if (*current_tile == Tile::Animal || connected_to_left.contains(current_tile))
            && connected_to_right.contains(next_tile)
        {
            neighbors.push(Position {
                y: position.y,
                x: position.x - 1,
            });
        }
    }
    if position.x + 1 < map.len() {
        let next_tile = &map[position.y][position.x + 1];
        if (*current_tile == Tile::Animal || connected_to_right.contains(current_tile))
            && connected_to_left.contains(next_tile)
        {
            neighbors.push(Position {
                y: position.y,
                x: position.x + 1,
            });
        }
    }
    neighbors
}
