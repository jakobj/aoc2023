use std::{error::Error, fmt, fs};

fn main() {
    let filename = "inputs/15.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let steps = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();
    let sum = steps.iter().map(|s| hash(s)).sum::<usize>();
    println!("The sum of the results is {sum}.");

    let instructions = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();
    let mut boxes = vec![Box(Vec::new()); 256];
    initialize(&instructions, &mut boxes);

    let focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.0.iter()
                .enumerate()
                .map(|(lens_idx, l)| (1 + box_idx) * (1 + lens_idx) * l.focal_length as usize)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("The focusing power of the resulting lens configuration is {focusing_power}.");
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc += c as u8 as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

#[derive(Debug)]
struct Instruction {
    label: String,
    op: Operation,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let chars = s.chars().collect::<Vec<char>>();
        match chars[chars.len() - 1] {
            '-' => Instruction {
                label: chars[..chars.len() - 1].iter().collect::<String>(),
                op: Operation::Remove,
            },
            _ => Instruction {
                label: chars[..chars.len() - 2].iter().collect::<String>(),
                op: Operation::Set(chars[chars.len() - 1].to_digit(10).unwrap() as u8),
            },
        }
    }
}

#[derive(Debug)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseInstructionError {}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Remove,
    Set(u8),
}

#[derive(Clone, Debug)]
struct Box(Vec<Lens>);

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn initialize(instructions: &[Instruction], boxes: &mut [Box]) {
    for ins in instructions.iter() {
        let lenses = &mut boxes[hash(&ins.label)].0;
        let lens_idx = lenses.iter().position(|l| l.label == ins.label);
        match ins.op {
            Operation::Remove => {
                if let Some(lens_idx) = lens_idx {
                    lenses.remove(lens_idx);
                };
            }
            Operation::Set(focal_length) => match lens_idx {
                Some(lens_idx) => lenses[lens_idx].focal_length = focal_length,
                None => lenses.push(Lens {
                    label: ins.label.clone(),
                    focal_length,
                }),
            },
        };
    }
}
