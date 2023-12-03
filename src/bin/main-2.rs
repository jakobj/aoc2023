use std::{collections::HashMap, fs, num::ParseIntError, str::FromStr};

fn main() {
    let filename = "inputs/2.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let games = content
        .lines()
        .map(|l| {
            l.parse::<Game>()
                .unwrap_or_else(|_| panic!("Could not convert line \"{l}\" to `Game`"))
        })
        .collect::<Vec<Game>>();

    let max_count = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let sum_of_ids = games
        .iter()
        .map(|g| {
            for r in g.rounds.iter() {
                for (color, count) in r.counts.iter() {
                    if *count > max_count[&color] {
                        return 0;
                    }
                }
            }
            g.id
        })
        .sum::<usize>();
    println!("The sum of the IDs of possible games is {sum_of_ids}.");

    let sum_of_powers = games.iter().map(
        |g| {
            let mut min_count = HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);
            for r in g.rounds.iter() {
                for (color, count) in r.counts.iter() {
                    min_count.insert(*color, std::cmp::max(min_count[color], *count));
                }
            }
            min_count.values().product::<usize>()
        }
    ).sum::<usize>();
    println!("The sum of the power of these sets is {sum_of_powers}.");
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, ParseGameError> {
        fn parse_id(s: &str) -> Result<usize, ParseGameError> {
            let mut s = s.split(' ');
            assert!(s.next().ok_or(ParseGameError)? == "Game");
            Ok(s.next().ok_or(ParseGameError)?.parse::<usize>()?)
        }

        fn parse_rounds(s: &str) -> Result<Vec<Round>, ParseGameError> {
            let mut rounds = Vec::new();
            for round_record in s.split("; ") {
                let mut round = Round::new();
                for count_record in round_record.split(", ") {
                    let count_record = count_record
                        .split(' ')
                        .skip_while(|&x| x.is_empty())
                        .collect::<Vec<&str>>();
                    assert!(count_record.len() == 2);
                    let count = count_record[0].parse::<usize>()?;
                    let color = count_record[1].parse::<Color>()?;
                    round.counts.insert(color, count);
                }
                rounds.push(round);
            }
            Ok(rounds)
        }

        let mut s = s.split(':');
        let id = parse_id(s.next().ok_or(ParseGameError)?)?;
        let rounds = parse_rounds(s.next().ok_or(ParseGameError)?)?;
        Ok(Game { id, rounds })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl From<ParseIntError> for ParseGameError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

#[derive(Debug)]
struct Round {
    pub counts: std::collections::HashMap<Color, usize>,
}

impl Round {
    fn new() -> Self {
        Self {
            counts: HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(Self::Err {}),
        }
    }
}
