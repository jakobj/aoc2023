use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn main() {
    let filename = "inputs/17.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");
    let grid = content
        .lines()
        .map(|l| l.chars().map(Block::from).collect::<Vec<Block>>())
        .collect::<Vec<Vec<Block>>>();

    let mut stack = BinaryHeap::new();
    stack.push(State {
        position: Position {
            y: 0,
            x: 0,
            n: 0,
            direction: Direction::Right,
        },
        loss: 0,
        trace: vec![],
    });
    stack.push(State {
        position: Position {
            y: 0,
            x: 0,
            n: 0,
            direction: Direction::Down,
        },
        loss: 0,
        trace: vec![],
    });
    let mut visited = HashSet::new();
    let mut min_loss = u32::MAX;
    let mut min_trace = vec![];
    while let Some(state) = stack.pop() {
        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position);

        if state.position.y + 1 == grid.len() as i32 && state.position.x + 1 == grid[0].len() as i32
        {
            min_loss = min_loss.min(state.loss);
            min_trace = state.trace.clone();
        }

        // go straight
        if state.position.n < 2 {
            let (y, x) = match state.position.direction {
                Direction::Up => (state.position.y - 1, state.position.x),
                Direction::Down => (state.position.y + 1, state.position.x),
                Direction::Left => (state.position.y, state.position.x - 1),
                Direction::Right => (state.position.y, state.position.x + 1),
            };
            if y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32 {
                let position = Position {
                    y,
                    x,
                    n: state.position.n + 1,
                    direction: state.position.direction,
                };
                let loss = state.loss + grid[y as usize][x as usize].0;
                let mut trace = state.trace.clone();
                trace.push((y, x, position.direction));
                stack.push(State {
                    position,
                    loss,
                    trace,
                });
            }
        }

        // turn left
        let (y, x, direction) = match state.position.direction {
            Direction::Up => (state.position.y, state.position.x - 1, Direction::Left),
            Direction::Down => (state.position.y, state.position.x + 1, Direction::Right),
            Direction::Left => (state.position.y + 1, state.position.x, Direction::Down),
            Direction::Right => (state.position.y - 1, state.position.x, Direction::Up),
        };
        if y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32 {
            let position = Position {
                y,
                x,
                n: 0,
                direction,
            };
            let loss = state.loss + grid[y as usize][x as usize].0;
            let mut trace = state.trace.clone();
            trace.push((y, x, position.direction));
            stack.push(State {
                position,
                loss,
                trace,
            });
        }

        // turn right
        let (y, x, direction) = match state.position.direction {
            Direction::Up => (state.position.y, state.position.x + 1, Direction::Right),
            Direction::Down => (state.position.y, state.position.x - 1, Direction::Left),
            Direction::Left => (state.position.y - 1, state.position.x, Direction::Up),
            Direction::Right => (state.position.y + 1, state.position.x, Direction::Down),
        };
        if y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[0].len() as i32 {
            let position = Position {
                y,
                x,
                n: 0,
                direction,
            };
            let loss = state.loss + grid[y as usize][x as usize].0;
            let mut trace = state.trace.clone();
            trace.push((y, x, position.direction));
            stack.push(State {
                position,
                loss,
                trace,
            });
        }
    }
    println!("The least heat loss the crucible can incur is {min_loss}.")
    // println!("{}", min_loss);
    // println!("{:?}", min_trace);
    // print(&grid, &min_trace);
}

#[derive(Clone, Copy, Debug)]
struct Block(u32);

impl From<char> for Block {
    fn from(c: char) -> Self {
        Self(c.to_digit(10).unwrap())
    }
}

#[derive(Clone, Eq)]
struct State {
    position: Position,
    trace: Vec<(i32, i32, Direction)>,
    loss: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // turn BinaryHeap into min-heap by reversing `self` and `other`
        other.loss.cmp(&self.loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.loss == other.loss
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    y: i32,
    x: i32,
    n: u8,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn print(grid: &[Vec<Block>], trace: &[(i32, i32, Direction)]) {
    println!();
    let mut new_grid = vec![vec![".".to_string(); grid[0].len()]; grid.len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, b) in row.iter().enumerate() {
            new_grid[i][j] = format!("{}", b.0);
        }
    }
    for pos in trace {
        new_grid[pos.0 as usize][pos.1 as usize] = pos.2.into();
    }

    for row in new_grid.iter() {
        for c in row.iter() {
            print!("{c}");
        }
        println!();
    }
}

impl From<Direction> for String {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}
