use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/10.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let mut map = parse_sketch(&content).expect("Should have been able to parse sketch");

    // bfs from animal position without repeated visits
    let animal_position = find_animal(&map).expect("Could not find animal position");
    let animal_tile = determine_animal_tile(&map, animal_position);
    map[animal_position.y][animal_position.x] = animal_tile;
    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(State {
        steps: 0,
        heuristic: 0,
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
                heuristic: 0,
                position: n,
            });
        }
    }
    println!("It takes {max_steps} steps along the loop to get from the starting position to the point farthest from the starting position.");

    // idea: subdivide each tile into nice subtiles, interpret main-loop pipes
    // as walls, do a shortest path search towards the sides of the map for each
    // (nonloop) tile
    let mut subtile_map = vec![vec![SubTile::Open; 3 * map[0].len()]; 3 * map.len()];
    for position in visited.iter() {
        let shape = map[position.y][position.x].get_subtile_shape();
        for dy in 0..3 {
            for dx in 0..3 {
                subtile_map[3 * position.y + dy][3 * position.x + dx] = shape[dy][dx];
            }
        }
    }

    let mut enclosed_positions = HashSet::new();
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let position = Position { y: i, x: j };
            if visited.contains(&position) {
                continue;
            }
            if !is_connected_to_boundary(&subtile_map, position, &enclosed_positions) {
                enclosed_positions.insert(position);
            }
        }
    }

    println!(
        "{} tiles are enclosed by the loop.",
        enclosed_positions.len()
    );
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Tile {
    fn is_connected_to(&self, other: &Self, relative_location: RelativeLocation) -> bool {
        let connected_to_up = HashSet::from([Tile::Vertical, Tile::NW, Tile::NE]);
        let connected_to_down = HashSet::from([Tile::Vertical, Tile::SW, Tile::SE]);
        let connected_to_left = HashSet::from([Tile::Horizontal, Tile::NW, Tile::SW]);
        let connected_to_right = HashSet::from([Tile::Horizontal, Tile::NE, Tile::SE]);

        assert!(*self != Tile::Animal);

        match relative_location {
            RelativeLocation::Up => {
                (connected_to_up.contains(self)) && connected_to_down.contains(other)
            }
            RelativeLocation::Down => {
                (connected_to_down.contains(self)) && connected_to_up.contains(other)
            }
            RelativeLocation::Left => {
                (connected_to_left.contains(self)) && connected_to_right.contains(other)
            }
            RelativeLocation::Right => {
                (connected_to_right.contains(self)) && connected_to_left.contains(other)
            }
        }
    }

    fn get_subtile_shape(&self) -> [[SubTile; 3]; 3] {
        match self {
            Self::Vertical => [[SubTile::Open, SubTile::Closed, SubTile::Open]; 3],
            Self::Horizontal => [
                [SubTile::Open, SubTile::Open, SubTile::Open],
                [SubTile::Closed, SubTile::Closed, SubTile::Closed],
                [SubTile::Open, SubTile::Open, SubTile::Open],
            ],
            Self::NE => [
                [SubTile::Open, SubTile::Closed, SubTile::Open],
                [SubTile::Open, SubTile::Closed, SubTile::Closed],
                [SubTile::Open, SubTile::Open, SubTile::Open],
            ],
            Self::NW => [
                [SubTile::Open, SubTile::Closed, SubTile::Open],
                [SubTile::Closed, SubTile::Closed, SubTile::Open],
                [SubTile::Open, SubTile::Open, SubTile::Open],
            ],
            Self::SW => [
                [SubTile::Open, SubTile::Open, SubTile::Open],
                [SubTile::Closed, SubTile::Closed, SubTile::Open],
                [SubTile::Open, SubTile::Closed, SubTile::Open],
            ],
            Self::SE => [
                [SubTile::Open, SubTile::Open, SubTile::Open],
                [SubTile::Open, SubTile::Closed, SubTile::Closed],
                [SubTile::Open, SubTile::Closed, SubTile::Open],
            ],
            Self::Ground => [
                [SubTile::Open, SubTile::Open, SubTile::Open],
                [SubTile::Open, SubTile::Open, SubTile::Open],
                [SubTile::Open, SubTile::Open, SubTile::Open],
            ],
            Self::Animal => panic!(r"who knows? ¯\_(ツ)_/¯"),
        }
    }
}

enum RelativeLocation {
    Up,
    Down,
    Left,
    Right,
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

fn determine_animal_tile(map: &[Vec<Tile>], position: Position) -> Tile {
    let connected_to_up = HashSet::from([Tile::Vertical, Tile::NW, Tile::NE]);
    let connected_to_down = HashSet::from([Tile::Vertical, Tile::SW, Tile::SE]);
    let connected_to_left = HashSet::from([Tile::Horizontal, Tile::NW, Tile::SW]);
    let connected_to_right = HashSet::from([Tile::Horizontal, Tile::NE, Tile::SE]);

    let up_tile = map[position.y - 1][position.x];
    let down_tile = map[position.y + 1][position.x];
    let left_tile = map[position.y][position.x - 1];
    let right_tile = map[position.y][position.x + 1];

    let xyz = [
        connected_to_down.contains(&up_tile),
        connected_to_left.contains(&right_tile),
        connected_to_up.contains(&down_tile),
        connected_to_right.contains(&left_tile),
    ];

    match xyz {
        [true, true, false, false] => Tile::NE,
        [true, false, true, false] => Tile::Vertical,
        [true, false, false, true] => Tile::NW,
        [false, true, true, false] => Tile::SE,
        [false, true, false, true] => Tile::Horizontal,
        [false, false, true, true] => Tile::SW,
        _ => unreachable!(),
    }
}

#[derive(Debug, Eq)]
struct State {
    steps: usize,
    heuristic: usize,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // turn it into a min-heap by switching order of `self` and `other`
        // (https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
        (other.steps + other.heuristic).cmp(&(self.steps + self.heuristic))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.heuristic == other.heuristic
    }
}

fn get_neighbors(map: &[Vec<Tile>], position: Position) -> Vec<Position> {
    let mut neighbors = Vec::new();

    let current_tile = &map[position.y][position.x];

    // move up
    if position.y > 0 {
        let next_tile = &map[position.y - 1][position.x];
        if current_tile.is_connected_to(next_tile, RelativeLocation::Up) {
            neighbors.push(Position {
                y: position.y - 1,
                x: position.x,
            });
        }
    }

    // move down
    if position.y + 1 < map.len() {
        let next_tile = &map[position.y + 1][position.x];
        if current_tile.is_connected_to(next_tile, RelativeLocation::Down) {
            neighbors.push(Position {
                y: position.y + 1,
                x: position.x,
            });
        }
    }

    // move left
    if position.x > 0 {
        let next_tile = &map[position.y][position.x - 1];
        if current_tile.is_connected_to(next_tile, RelativeLocation::Left) {
            neighbors.push(Position {
                y: position.y,
                x: position.x - 1,
            });
        }
    }

    // move right
    if position.x + 1 < map.len() {
        let next_tile = &map[position.y][position.x + 1];
        if current_tile.is_connected_to(next_tile, RelativeLocation::Right) {
            neighbors.push(Position {
                y: position.y,
                x: position.x + 1,
            });
        }
    }

    neighbors
}

fn is_connected_to_boundary(
    map: &Vec<Vec<SubTile>>,
    position: Position,
    enclosed_positions: &HashSet<Position>,
) -> bool {
    let mut stack = BinaryHeap::new();
    let mut visited = HashSet::new();
    stack.push(State {
        steps: 0,
        heuristic: compute_heuristic(map, position),
        position: Position {
            y: 3 * position.y,
            x: 3 * position.x,
        },
    });
    while let Some(current_state) = stack.pop() {
        if visited.contains(&current_state.position) {
            continue;
        }
        visited.insert(current_state.position);

        if current_state.position.y == 0
            || current_state.position.y + 1 == map.len()
            || current_state.position.x == 0
            || current_state.position.x + 1 == map[0].len()
        {
            return true;
        }

        // if we encounter an enclosed position, we know that there's NO WAY
        // OUT! since `enclosed_positions` is not storing subtile positions, we
        // first need to "coarse grain" again!
        let tile_position = Position {
            y: current_state.position.y / 3,
            x: current_state.position.x / 3,
        };
        if enclosed_positions.contains(&tile_position) {
            return false;
        }

        let neighbors = get_subtile_neighbors(map, current_state.position);
        for position in neighbors {
            stack.push(State {
                steps: current_state.steps + 1,
                heuristic: compute_heuristic(map, position),
                position,
            });
        }
    }
    false
}

fn compute_heuristic(map: &Vec<Vec<SubTile>>, position: Position) -> usize {
    [
        position.y,
        position.x,
        map.len() - position.y,
        map[0].len() - position.x,
    ]
    .into_iter()
    .min()
    .unwrap()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SubTile {
    Open,
    Closed,
}

fn get_subtile_neighbors(map: &[Vec<SubTile>], position: Position) -> Vec<Position> {
    let mut neighbors = Vec::new();
    if position.y > 0 && map[position.y - 1][position.x] != SubTile::Closed {
        neighbors.push(Position {
            y: position.y - 1,
            x: position.x,
        });
    }
    if position.y + 1 < map.len() && map[position.y + 1][position.x] != SubTile::Closed {
        neighbors.push(Position {
            y: position.y + 1,
            x: position.x,
        });
    }
    if position.x > 0 && map[position.y][position.x - 1] != SubTile::Closed {
        neighbors.push(Position {
            y: position.y,
            x: position.x - 1,
        });
    }
    if position.x + 1 < map[0].len() && map[position.y][position.x + 1] != SubTile::Closed {
        neighbors.push(Position {
            y: position.y,
            x: position.x + 1,
        });
    }
    neighbors
}
