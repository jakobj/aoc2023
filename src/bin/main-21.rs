use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/21.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let map = content
        .lines()
        .map(|l| l.chars().map(Tile::from).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();
    let start = find_start(&map);

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut counter = 0;
    queue.push(State {
        position: Position {
            y: start.0,
            x: start.1,
        },
        steps: 0,
    });
    while let Some(state) = queue.pop() {
        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position);

        if state.steps > 64 {
            break;
        }

        // can always reach the same spot again in even number of steps
        if state.steps % 2 == 0 {
            counter += 1;
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let y = state.position.y as i32 + dy;
            let x = state.position.x as i32 + dx;
            if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[0].len() as i32 {
                let y = y as usize;
                let x = x as usize;
                if map[y][x] != Tile::Rock {
                    queue.push(State {
                        position: Position { y, x },
                        steps: state.steps + 1,
                    });
                }
            }
        }
    }
    println!("The elf could reach {counter} garden plots in exactly 64 steps.");
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Start,
    GardenPlot,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            '.' => Tile::GardenPlot,
            '#' => Tile::Rock,
            _ => panic!("Error while parsing tile '{c}'"),
        }
    }
}

fn find_start(map: &[Vec<Tile>]) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if tile == Tile::Start {
                return (i, j);
            }
        }
    }
    unreachable!();
}

#[derive(Clone, Copy, Debug, Eq)]
struct State {
    position: Position,
    steps: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    y: usize,
    x: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // turn it into a min-heap by switching order of `self` and `other`
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
