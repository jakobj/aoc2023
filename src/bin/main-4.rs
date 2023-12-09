use std::{fs, num::ParseIntError, str::FromStr};

fn main() {
    let filename = "inputs/4.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let worth = content
        .lines()
        .map(|l| {
            let v = l.split(':').collect::<Vec<&str>>();
            let v = v[1].split('|').collect::<Vec<&str>>();
            let winnings_numbers = v[0].parse::<Numbers>().unwrap();
            let numbers = v[1].parse::<Numbers>().unwrap();

            let count = numbers
                .numbers
                .iter()
                .filter(|&&n| winnings_numbers.numbers.iter().any(|&wn| wn == n))
                .map(|_| 1)
                .sum::<u32>();

            if count == 0 {
                0
            } else {
                2_usize.pow(count - 1)
            }
        })
        .sum::<usize>();

    println!("The colorful cards are {worth} worth in total.")
}

#[derive(Debug)]
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
