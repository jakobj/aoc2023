use std::fs;

fn main() {
    let filename = "inputs/3.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let data = content
        .lines()
        .map(|l| l.chars().map(Element::from).collect::<Vec<Element>>())
        .collect::<Vec<Vec<Element>>>();
    let numbers = extract_numbers(data);
    let sum = numbers
        .iter()
        .filter(|n| {
            n.neighbors
                .iter()
                .any(|e| *e == Element::Symbol || *e == Element::Gear)
        })
        .map(|n| n.value)
        .sum::<usize>();
    println!("The sum of all of the part numbers in the engine schematic is {sum}.");

    let mut prod = 0;
    'outer: for (i, ni) in numbers.iter().enumerate() {
        for nj in numbers.iter().skip(i + 1) {
            for gi in ni.gears.iter() {
                for gj in nj.gears.iter() {
                    if gi.position == gj.position {
                        prod += ni.value * nj.value;
                        continue 'outer;
                    }
                }
            }
        }
    }
    println!("The sum of all of the gear ratios in your engine schematic is {prod}.");
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Element {
    Digit(usize),
    Period,
    Gear,
    Symbol,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        if let Some(v) = value.to_digit(10) {
            Element::Digit(v as usize)
        } else if value == '.' {
            Element::Period
        } else if value == '*' {
            Element::Gear
        } else {
            Element::Symbol
        }
    }
}

fn extract_numbers(data: Vec<Vec<Element>>) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut stack = Vec::new();
    let mut neighbors = Vec::new();
    let mut gears = Vec::new();
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
                                let k = k as usize;
                                let l = l as usize;
                                let n = data[k][l];
                                neighbors.push(n);
                                if n == Element::Gear {
                                    gears.push(Gear { position: (k, l) });
                                }
                            }
                        }
                    }
                }
                Element::Period | Element::Symbol | Element::Gear => {
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
                        gears: gears.clone(),
                    });
                    stack.clear();
                    neighbors.clear();
                    gears.clear();
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
    gears: Vec<Gear>,
}

#[derive(Clone, Copy, Debug)]
struct Gear {
    position: (usize, usize),
}
