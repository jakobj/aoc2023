use std::{error::Error, fmt, fs};

fn main() {
    let filename = "inputs/13.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let patterns = parse_patterns(&content).expect("Should have been able to parse patterns");
    let summary = patterns.iter().map(find_reflection).sum::<usize>();
    println!("You get {summary} after summarizing all of your notes.");
}

fn parse_patterns(s: &str) -> Result<Vec<Pattern>, ParseGroundError> {
    let mut patterns = vec![Pattern(Vec::new())];
    for l in s.lines() {
        if !l.is_empty() {
            patterns.last_mut().unwrap().0.push(
                l.chars()
                    .map(Ground::try_from)
                    .collect::<Result<Vec<Ground>, ParseGroundError>>()?,
            );
        } else {
            patterns.push(Pattern(Vec::new()));
        }
    }
    Ok(patterns)
}

#[derive(Clone, Debug)]
struct Pattern(Vec<Vec<Ground>>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Ground {
    Ash,
    Rock,
}

impl TryFrom<char> for Ground {
    type Error = ParseGroundError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Ground::Ash),
            '#' => Ok(Ground::Rock),
            _ => Err(ParseGroundError),
        }
    }
}

#[derive(Debug)]
struct ParseGroundError;

impl fmt::Display for ParseGroundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseGroundError {}

fn find_reflection(p: &Pattern) -> usize {
    // search rows
    'rows: for (i, row) in p.0.iter().enumerate().take(p.0.len() - 1) {
        'offset: for offset in 0..(i + 1) {
            if i < offset || i + 1 + offset >= p.0.len() {
                break 'offset;
            }
            for j in 0..row.len() {
                if p.0[i - offset][j] != p.0[i + 1 + offset][j] {
                    continue 'rows;
                }
            }
        }
        return (i + 1) * 100;
    }

    // search cols
    'cols: for j in 0..(p.0[0].len() - 1) {
        'offset: for offset in 0..(j + 1) {
            if j < offset || j + 1 + offset >= p.0[0].len() {
                break 'offset;
            }
            for i in 0..p.0.len() {
                if p.0[i][j - offset] != p.0[i][j + 1 + offset] {
                    continue 'cols;
                }
            }
        }
        return j + 1;
    }

    panic!("No reflection found");
}
