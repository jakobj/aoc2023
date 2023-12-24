use std::{collections::HashMap, error::Error, fmt, fs, iter, num::ParseIntError};

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
        .map(|r| {
            let mut lut = HashMap::new();
            count_arrangements(&r.conditions, &r.summary, &mut lut)
        })
        .sum::<usize>();
    println!("The sum of those counts is {n_arrangements}.");

    let full_records = records.iter().map(unfold_record).collect::<Vec<Record>>();
    let n_new_arrangements = full_records
        .iter()
        .map(|r| {
            let mut lut = HashMap::new();
            count_arrangements(&r.conditions, &r.summary, &mut lut)
        })
        .sum::<usize>();
    println!("The new sum of those counts is {n_new_arrangements}.");
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

// fn to_string(conditions: &[Condition]) -> String {
//     conditions
//         .iter()
//         .map(|c| format!("{}", c))
//         .collect::<String>()
// }

// impl fmt::Display for Condition {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Condition::Operational => write!(f, "."),
//             Condition::Damaged => write!(f, "#"),
//             Condition::Unknown => write!(f, "?"),
//         }
//     }
// }

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

fn count_arrangements(
    conditions: &[Condition],
    summary: &[usize],
    lut: &mut HashMap<State, usize>,
) -> usize {
    // prune branches via various strategies
    // 1) we already have too many damaged springs
    let n_damaged = count_damaged(conditions);
    if n_damaged > summary.iter().sum::<usize>() {
        return 0;
    }
    // 2) we already have too many operational springs
    if n_damaged + count_unknown(conditions) < summary.iter().sum::<usize>() {
        return 0;
    }
    let preliminary_summary = compute_preliminary_summary(conditions);
    // 3) more damaged groups than target
    if preliminary_summary.len() > summary.len() {
        return 0;
    }
    // 4) damaged groups so far don't match the target
    if !preliminary_summary.is_empty() {
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

    // if we reach here, we're still on the right track!

    // if there's nothing to replace, check validity
    if !conditions.iter().any(|&c| c == Condition::Unknown) {
        if compute_summary(conditions) == summary {
            return 1;
        } else {
            return 0;
        }
    }

    for (i, &c) in conditions.iter().enumerate() {
        if c == Condition::Unknown {
            let state = State {
                idx: i,
                preliminary_summary,
                previous_condition: if i > 0 {
                    conditions[i - 1]
                } else {
                    Condition::Unknown // doesn't really matter, but `Unknown`
                                       // seems accurate ;)
                },
            };

            // we know what's coming next
            if lut.contains_key(&state) {
                return lut[&state];
            }

            // if we don't know, we need to count
            let mut new_conditions = conditions.to_vec();
            let mut n_arrangements = 0;
            new_conditions[i] = Condition::Damaged;
            n_arrangements += count_arrangements(&new_conditions, summary, lut);
            new_conditions[i] = Condition::Operational;
            n_arrangements += count_arrangements(&new_conditions, summary, lut);
            lut.insert(state, n_arrangements);
            return n_arrangements; // we break early, replacing `Unknown`s from left to right
        }
    }
    0
}

fn unfold_record(record: &Record) -> Record {
    Record {
        conditions: iter::repeat(&record.conditions)
            .take(5)
            .flat_map(|v| {
                let mut v = v.to_vec();
                v.insert(0, Condition::Unknown);
                v
            })
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

#[derive(Debug, Eq, Hash, PartialEq)]
struct State {
    idx: usize,
    preliminary_summary: Vec<usize>,
    previous_condition: Condition,
}
