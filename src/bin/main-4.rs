use std::{fs, num::ParseIntError, str::FromStr};

fn main() {
    let filename = "inputs/4.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let worth = content
        .lines()
        .map(|l| {
            let (winning_numbers, numbers) = parse_line(l);
            let matches = count_matches(winning_numbers, numbers);
            if matches == 0 {
                0
            } else {
                2_usize.pow(matches as u32 - 1)
            }
        })
        .sum::<usize>();

    println!("The colorful cards are {worth} worth in total.");

    let originals = content
        .lines()
        .map(parse_line)
        .enumerate()
        .collect::<Vec<(usize, (Numbers, Numbers))>>();
    let mut count = originals.len();
    let mut stack = originals.clone();
    while let Some((offset, (winning_numbers, numbers))) = stack.pop() {
        let matches = count_matches(winning_numbers, numbers);
        for i in 0..matches {
            stack.push(originals[offset + 1 + i].clone());
            count += 1;
        }
    }
    println!("In total you end up with {count} scratchcards.");
}

#[derive(Clone, Debug)]
struct Numbers {
    numbers: Vec<usize>,
}

impl FromStr for Numbers {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(' ')
            .filter(|si| !si.is_empty())
            .map(|si| si.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Ok(Self { numbers })
    }
}

fn parse_line(l: &str) -> (Numbers, Numbers) {
    let v = l.split(':').collect::<Vec<&str>>();
    let v = v[1].split('|').collect::<Vec<&str>>();
    (
        v[0].parse::<Numbers>().unwrap(),
        v[1].parse::<Numbers>().unwrap(),
    )
}

fn count_matches(winning_numbers: Numbers, numbers: Numbers) -> usize {
    numbers
        .numbers
        .iter()
        .filter(|&&n| winning_numbers.numbers.iter().any(|&wn| wn == n))
        .count()
}
