use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/8.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let mut content = content.lines().collect::<Vec<&str>>();
    let instructions = parse_instructions(content[0]).unwrap();
    let network = parse_network(&content.split_off(2));

    let steps = count_steps(&network, &instructions);
    println!("{steps} steps are required to reach ZZZ.");

    let ghost_steps = count_ghost_steps(&network, &instructions);
    println!("{ghost_steps} ghost steps are required to reach **Z.");
}

fn parse_instructions(s: &str) -> Result<Vec<Direction>, ParseDirectionError> {
    s.chars()
        .map(Direction::try_from)
        .collect::<Result<Vec<Direction>, ParseDirectionError>>()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(Self::Error {}),
        }
    }
}

#[derive(Debug)]
struct ParseDirectionError {}

fn parse_network(s: &Vec<&str>) -> HashMap<Node, (Node, Node)> {
    let mut network = HashMap::new();
    for l in s {
        let l = l.split(" = ").collect::<Vec<&str>>();
        let key = l[0];
        let l = l[1][1..l[1].len() - 1].split(", ").collect::<Vec<&str>>();
        let (left, right) = (l[0], l[1]);
        network.insert(Node::from(key), (Node::from(left), Node::from(right)));
    }
    network
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Node {
    key: String,
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        Node { key: s.to_string() }
    }
}

fn count_steps(network: &HashMap<Node, (Node, Node)>, instructions: &Vec<Direction>) -> usize {
    let mut count = 0;
    let mut current_node = &Node::from("AAA");
    while *current_node != Node::from("ZZZ") {
        current_node = match instructions[count % instructions.len()] {
            Direction::Left => &network[&current_node].0,
            Direction::Right => &network[&current_node].1,
        };
        count += 1;
    }
    count
}

fn count_ghost_steps(network: &HashMap<Node, (Node, Node)>, instructions: &[Direction]) -> usize {
    // idea: find periods for each node ending in 'A', then find least common
    // multiple
    let current_nodes = network
        .keys()
        .filter(|n| n.key.ends_with('A'))
        .cloned()
        .collect::<Vec<Node>>();
    let periods = current_nodes
        .iter()
        .map(|n| determine_period(network, instructions, n))
        .collect::<Vec<usize>>();
    find_lcm(&periods)
}

fn determine_period(
    network: &HashMap<Node, (Node, Node)>,
    instructions: &[Direction],
    starting_node: &Node,
) -> usize {
    // count steps until encountering first node ending in Z
    let mut count = 0;
    let mut current_node = starting_node;
    while !current_node.key.ends_with('Z') {
        current_node = match instructions[count % instructions.len()] {
            Direction::Left => &network[&current_node].0,
            Direction::Right => &network[&current_node].1,
        };
        count += 1;
    }
    let end_node = current_node;
    let period = count;

    // make sure that one ends up at the same node after `period` number of
    // steps
    for _ in 0..period {
        current_node = match instructions[count % instructions.len()] {
            Direction::Left => &network[&current_node].0,
            Direction::Right => &network[&current_node].1,
        };
        count += 1;
    }
    assert!(current_node == end_node);

    period
}

fn find_lcm(x: &[usize]) -> usize {
    let mut lcm = x[0];
    for item in x.iter().skip(1) {
        lcm = compute_lcm(lcm, *item);
    }
    lcm
}

fn compute_lcm(a: usize, b: usize) -> usize {
    let gcd = compute_gcd(a, b);
    a * (b / gcd)
}

fn compute_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
