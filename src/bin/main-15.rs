use std::fs;

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
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc += c as u8 as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}
