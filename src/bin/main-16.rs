use std::{collections::HashSet, fs};

fn main() {
    let filename = "inputs/16.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let grid = content
        .lines()
        .map(|l| l.chars().map(Tile::from).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let energized_tiles = count_energized_tiles(&grid, Position { y: 0, x: 0 }, Direction::Right);
    println!("{energized_tiles} tiles end up being energized.");

    let binding = [
        (0..grid[0].len())
            .map(|x| count_energized_tiles(&grid, Position { y: 0, x: x as i32 }, Direction::Down))
            .max()
            .unwrap(),
        (0..grid[0].len())
            .map(|x| count_energized_tiles(&grid, Position {y: grid.len() as i32 - 1, x: x as i32,}, Direction::Up))
            .max()
            .unwrap(),
        (0..grid[0].len())
            .map(|y| count_energized_tiles(&grid, Position { y: y as i32, x: 0 }, Direction::Right))
            .max()
            .unwrap(),
        (0..grid[0].len())
            .map(|y| count_energized_tiles(&grid, Position {y: y as i32, x: grid[0].len() as i32 - 1,}, Direction::Left))
            .max()
            .unwrap(),
    ];
    let most_energized_tiles = binding.iter().max().unwrap();
    println!("The largest number of energized tiles is {most_energized_tiles}.")
}

fn move_up(state: State) -> State {
    State {
        position: Position {
            y: state.position.y - 1,
            x: state.position.x,
        },
        direction: Direction::Up,
    }
}

fn move_down(state: State) -> State {
    State {
        position: Position {
            y: state.position.y + 1,
            x: state.position.x,
        },
        direction: Direction::Down,
    }
}

fn move_left(state: State) -> State {
    State {
        position: Position {
            y: state.position.y,
            x: state.position.x - 1,
        },
        direction: Direction::Left,
    }
}

fn move_right(state: State) -> State {
    State {
        position: Position {
            y: state.position.y,
            x: state.position.x + 1,
        },
        direction: Direction::Right,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    SlashMirror,
    BackSlashMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::SlashMirror,
            '\\' => Self::BackSlashMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!("Could not parse tile '{c}'"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    y: i32,
    x: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn count_energized_tiles(
    grid: &[Vec<Tile>],
    starting_position: Position,
    starting_direction: Direction,
) -> usize {
    let mut stack = Vec::new();
    stack.push(State {
        position: starting_position,
        direction: starting_direction,
    });
    let mut visited = HashSet::new();
    while let Some(state) = stack.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        let tile = grid[state.position.y as usize][state.position.x as usize];
        let mut potential_states = Vec::new();
        match state.direction {
            Direction::Up => {
                if tile == Tile::Empty || tile == Tile::VerticalSplitter {
                    potential_states.push(move_up(state));
                }
                if tile == Tile::SlashMirror || tile == Tile::HorizontalSplitter {
                    potential_states.push(move_right(state));
                }
                if tile == Tile::BackSlashMirror || tile == Tile::HorizontalSplitter {
                    potential_states.push(move_left(state));
                }
            }
            Direction::Down => {
                if tile == Tile::Empty || tile == Tile::VerticalSplitter {
                    potential_states.push(move_down(state));
                }
                if tile == Tile::SlashMirror || tile == Tile::HorizontalSplitter {
                    potential_states.push(move_left(state));
                }
                if tile == Tile::BackSlashMirror || tile == Tile::HorizontalSplitter {
                    potential_states.push(move_right(state));
                }
            }
            Direction::Left => {
                if tile == Tile::Empty || tile == Tile::HorizontalSplitter {
                    potential_states.push(move_left(state));
                }
                if tile == Tile::SlashMirror || tile == Tile::VerticalSplitter {
                    potential_states.push(move_down(state));
                }
                if tile == Tile::BackSlashMirror || tile == Tile::VerticalSplitter {
                    potential_states.push(move_up(state));
                }
            }
            Direction::Right => {
                if tile == Tile::Empty || tile == Tile::HorizontalSplitter {
                    potential_states.push(move_right(state));
                }
                if tile == Tile::SlashMirror || tile == Tile::VerticalSplitter {
                    potential_states.push(move_up(state));
                }
                if tile == Tile::BackSlashMirror || tile == Tile::VerticalSplitter {
                    potential_states.push(move_down(state));
                }
            }
        }
        for state in potential_states.into_iter() {
            if state.position.y >= 0
                && state.position.y < grid.len() as i32
                && state.position.x >= 0
                && state.position.x < grid[0].len() as i32
            {
                stack.push(state);
            }
        }
    }

    visited
        .into_iter()
        .map(|s| s.position)
        .collect::<HashSet<Position>>()
        .len()
}
