use std::fs;

fn main() {
    let filename = "inputs/3.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let data = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Element::from(c))
                .collect::<Vec<Element>>()
        })
        .collect::<Vec<Vec<Element>>>();
    let numbers = extract_numbers(data);
    let sum = numbers
        .iter()
        .filter(|n| n.neighbors.iter().any(|e| *e == Element::Symbol))
        .map(|n| n.value)
        .sum::<usize>();
    println!("The sum of all of the part numbers in the engine schematic is {sum}.")
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Element {
    Digit(usize),
    Period,
    Symbol,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        if let Some(v) = value.to_digit(10) {
            Element::Digit(v as usize)
        } else if value == '.' {
            Element::Period
        } else {
            Element::Symbol
        }
    }
}

fn extract_numbers(data: Vec<Vec<Element>>) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut stack = Vec::new();
    let mut neighbors = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            match data[i][j] {
                Element::Digit(v) => {
                    stack.push(v);
                    for di in [-1, 0, 1] {
                        for dj in [-1, 0, 1] {
                            let k = i as i32 + di;
                            let l = j as i32 + dj;
                            if k >= 0 && k < data.len() as i32 && l >= 0 && l < data[0].len() as i32
                            {
                                neighbors.push(data[k as usize][l as usize]);
                            }
                        }
                    }
                }
                Element::Period | Element::Symbol => {
                    if stack.is_empty() {
                        continue;
                    }
                    let mut value = 0;
                    let mut base = 1;
                    while let Some(d) = stack.pop() {
                        value += d * base;
                        base *= 10;
                    }
                    numbers.push(Number {
                        value,
                        neighbors: neighbors.clone(),
                    });
                    stack.clear();
                    neighbors.clear();
                }
            }
        }
    }
    numbers
}

#[derive(Debug)]
struct Number {
    value: usize,
    neighbors: Vec<Element>,
}
