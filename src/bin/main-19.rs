use std::{collections::HashMap, fs};

fn main() {
    let filename = "inputs/19.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let workflows = content
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let l = l.split('{').collect::<Vec<&str>>();
            let name = l[0].to_string();
            let rules_str = l[1][..l[1].len() - 1].split(',').collect::<Vec<&str>>();
            let workflow = rules_str.into_iter().map(Rule::from).collect::<Vec<Rule>>();
            (name, workflow)
        })
        .collect::<HashMap<String, Vec<Rule>>>();

    let parts = content
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| Part::from(&l[1..l.len() - 1]))
        .collect::<Vec<Part>>();

    let mut accepted_parts = Vec::new();
    for p in parts.iter() {
        if apply_rules(p, "in", &workflows) {
            accepted_parts.push(p.clone());
        }
    }
    let sum = accepted_parts
        .into_iter()
        .map(|p| p.ratings.iter().sum::<usize>())
        .sum::<usize>();
    println!("If you add together all of the rating numbers for all of the parts that ultimately get accepted, you get {sum}.");
}

#[derive(Debug)]
struct Rule {
    category: Option<Category>,
    condition: Option<Condition>,
    rating: Option<usize>,
    destination: String,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        if s.find(':').is_some() {
            let category = Some(Category::from(&s[0..1]));
            let condition = Some(Condition::from(&s[1..2]));
            let s = s[2..].split(':').collect::<Vec<&str>>();
            let rating = Some(s[0].parse::<usize>().unwrap());
            let destination = s[1].to_string();
            Self {
                category,
                condition,
                rating,
                destination,
            }
        } else {
            let destination = s.to_string();
            Self {
                category: None,
                condition: None,
                rating: None,
                destination,
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        assert!(s.len() == 1);
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Error while parsing category '{s}'"),
        }
    }
}

impl Category {
    fn to_index(self) -> usize {
        match self {
            Self::X => 0,
            Self::M => 1,
            Self::A => 2,
            Self::S => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    Less,
    Greater,
}

impl From<&str> for Condition {
    fn from(s: &str) -> Self {
        assert!(s.len() == 1);
        match s {
            "<" => Self::Less,
            ">" => Self::Greater,
            _ => panic!("Error while parsing condition '{s}'"),
        }
    }
}

#[derive(Clone, Debug)]
struct Part {
    ratings: Vec<usize>,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let mut ratings = Vec::new();
        for si in s.split(',') {
            let si = si.split('=').collect::<Vec<&str>>();
            ratings.push(si[1].parse::<usize>().unwrap());
        }
        Self { ratings }
    }
}

fn apply_rules(part: &Part, destination: &str, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    if destination == "A" {
        return true;
    }

    if destination == "R" {
        return false;
    }

    for r in workflows[destination].iter() {
        match (r.condition, r.category, r.rating) {
            (Some(Condition::Less), Some(category), Some(rating)) => {
                if part.ratings[category.to_index()] < rating {
                    return apply_rules(part, &r.destination, workflows);
                }
            }
            (Some(Condition::Greater), Some(category), Some(rating)) => {
                if part.ratings[category.to_index()] > rating {
                    return apply_rules(part, &r.destination, workflows);
                }
            }
            (None, None, None) => return apply_rules(part, &r.destination, workflows),
            _ => unreachable!(),
        }
    }
    unreachable!();
}
