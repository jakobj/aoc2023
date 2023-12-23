use std::{error::Error, fmt, fs, num::ParseIntError};

fn main() {
    let filename = "inputs/12.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let records = content
        .lines()
        .map(|l| {
            let conditions = l
                .chars()
                .take_while(|&c| c != ' ')
                .map(Condition::try_from)
                .collect::<Result<Vec<Condition>, ConditionParseError>>()
                .expect("Should have been able to extract conditions");
            let summary = l
                .chars()
                .skip_while(|&c| c != ' ')
                .skip(1)
                .collect::<String>()
                .split(',')
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()
                .expect("Should have been able to extract summary");
            Record {
                conditions,
                summary,
            }
        })
        .collect::<Vec<Record>>();

    let n_arrangements = records
        .iter()
        .map(|r| count_arrangements(&r.conditions, &r.summary))
        .sum::<usize>();
    println!("The sum of those counts is {n_arrangements}.");
}

fn to_string(conditions: &[Condition]) -> String {
    conditions
        .iter()
        .map(|c| format!("{}", c))
        .collect::<String>()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Operational => write!(f, "."),
            Condition::Damaged => write!(f, "#"),
            Condition::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for Condition {
    type Error = ConditionParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err(ConditionParseError),
        }
    }
}

#[derive(Debug)]
struct ConditionParseError;

impl fmt::Display for ConditionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ConditionParseError {}

#[derive(Clone, Debug)]
struct Record {
    conditions: Vec<Condition>,
    summary: Vec<usize>,
}

fn count_arrangements(conditions: &[Condition], summary: &[usize]) -> usize {
    // if there's nothing to replace, check validity
    if !conditions.iter().any(|&c| c == Condition::Unknown) {
        if conditions
            .iter()
            .filter(|&&c| c == Condition::Damaged)
            .count()
            != summary.iter().sum::<usize>()
        {
            return 0;
        }
        let actual_summary = conditions
            .split(|&c| c == Condition::Operational)
            .map(|s| s.len())
            .filter(|&n| n > 0)
            .collect::<Vec<usize>>();
        if actual_summary == summary {
            return 1;
        } else {
            return 0;
        }
    }

    let mut n_arrangements = 0;
    for (i, &c) in conditions.iter().enumerate() {
        if c == Condition::Unknown {
            let mut new_conditions = conditions.to_vec();
            new_conditions[i] = Condition::Damaged;
            n_arrangements += count_arrangements(&new_conditions, summary);
            new_conditions[i] = Condition::Operational;
            n_arrangements += count_arrangements(&new_conditions, summary);
            break; // we break early, replacing `Unknown`s from left to right
        }
    }
    n_arrangements
}
