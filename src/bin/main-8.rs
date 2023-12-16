use std::{fs, collections::HashMap};

fn main() {
    let filename = "inputs/8.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let mut content = content.lines().collect::<Vec<&str>>();
    let instructions = parse_instructions(content[0]).unwrap();
    let network = parse_network(&content.split_off(2));
    let steps = count_steps(&network, &instructions);
    println!("{steps} steps are required to reach ZZZ.");
}

fn parse_instructions(s: &str) -> Result<Vec<Direction>, ParseDirectionError> {
    s.chars().map(Direction::try_from).collect::<Result<Vec<Direction>, ParseDirectionError>>()
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
            _ => Err(Self::Error{})
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
        Node{ key: s.to_string() }
    }
}

fn count_steps(network: &HashMap<Node, (Node, Node)>, instructions: &Vec<Direction>) -> usize {
    let mut count = 0;
    let mut current_position = Node::from("AAA");
    while current_position != Node::from("ZZZ") {
        current_position = match instructions[count % instructions.len()] {
            Direction::Left => network[&current_position].0.clone(),
            Direction::Right => network[&current_position].1.clone(),
        };
        count += 1;
    }
    count
}
