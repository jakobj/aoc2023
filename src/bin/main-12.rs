use std::{error::Error, fmt, fs, iter, num::ParseIntError};

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

    let full_records = records.iter().map(unfold_record).collect::<Vec<Record>>();
    let n_new_arrangements = full_records
        .iter()
        .enumerate()
        .skip(1)
        .take(1)
        .map(|(i, r)| {
            count_arrangements(&r.conditions, &r.summary)
        })
        //     .sum::<usize>();
        // println!("The new sum of those counts is {n_new_arrangements}.");
        .collect::<Vec<usize>>();
    println!("{:?}", n_new_arrangements);
    println!("{}", n_new_arrangements.iter().sum::<usize>());
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
    // prune branches via various strategies
    // 1) we already have too many damaged springs
    let n_damaged = count_damaged(&conditions);
    if n_damaged > summary.iter().sum::<usize>() {
        return 0;
    }
    // 2) we already have too many operational springs
    if n_damaged + count_unknown(&conditions) < summary.iter().sum::<usize>() {
        return 0;
    }
    let preliminary_summary = compute_preliminary_summary(conditions);
    // 3) more damaged groups than target
    if preliminary_summary.len() > summary.len() {
        return 0;
    }
    // 4) damaged groups so far don't match the target
    if preliminary_summary.len() > 0 {
        if preliminary_summary[preliminary_summary.len() - 1]
            > summary[preliminary_summary.len() - 1]
        {
            return 0;
        }
        for (actual_count, target_count) in preliminary_summary
            .iter()
            .take(preliminary_summary.len() - 1)
            .zip(summary.iter())
        {
            if actual_count != target_count {
                return 0;
            }
        }
    }
    // println!("{} {:?} {:?}", to_string(conditions), preliminary_summary, summary);

    // if we reach here, we're still on the right track!

    // if there's nothing to replace, check validity
    if !conditions.iter().any(|&c| c == Condition::Unknown) {
        if compute_summary(conditions) == summary {
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

fn unfold_record(record: &Record) -> Record {
    Record {
        conditions: iter::repeat(&record.conditions)
            .take(5)
            .map(|v| {
                let mut v = v.to_vec();
                v.insert(0, Condition::Unknown);
                v
            })
            .flatten()
            .skip(1) // we should only insert '?' *between* lists
            .collect::<Vec<Condition>>(),
        summary: iter::repeat(&record.summary)
            .take(5)
            .flatten()
            .copied()
            .collect::<Vec<usize>>(),
    }
}

fn count_damaged(conditions: &[Condition]) -> usize {
    conditions
        .iter()
        .filter(|&&c| c == Condition::Damaged)
        .count()
}

fn count_unknown(conditions: &[Condition]) -> usize {
    conditions
        .iter()
        .filter(|&&c| c == Condition::Unknown)
        .count()
}

fn compute_summary(conditions: &[Condition]) -> Vec<usize> {
    conditions
        .split(|&c| c == Condition::Operational)
        .map(|s| s.len())
        .filter(|&n| n > 0)
        .collect::<Vec<usize>>()
}

fn compute_preliminary_summary(conditions: &[Condition]) -> Vec<usize> {
    let conditions = conditions
        .iter()
        .take_while(|&&c| c != Condition::Unknown)
        .copied()
        .collect::<Vec<Condition>>();
    compute_summary(&conditions)
}
