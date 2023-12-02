use std::fs;

static DIGITS: &[(&str, &str)] = &[
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn main() {
    let filename = "inputs/1.txt";
    let content = fs::read_to_string(filename).unwrap();

    let sum = compute_sum_of_calibration_values(&content, false);
    println!("The sum of all of the calibration values is {}.", sum);

    let sum = compute_sum_of_calibration_values(&content, true);
    println!(
        "The correct sum of all of the calibration values is {}.",
        sum
    );
}

fn compute_sum_of_calibration_values(content: &str, replace: bool) -> u32 {
    let mut sum = 0;
    for l in content.lines() {
        let mut first = None;
        let mut last = None;

        let lc = if replace {
            replace_first_spelled_out_digit(l)
        } else {
            l.to_owned()
        };
        for c in lc.chars() {
            if c.is_numeric() {
                first = Some(c);
                last = Some(c);
                break; // once we found the first one, we're done
            }
        }

        let lc = if replace {
            replace_last_spelled_out_digit(l)
        } else {
            l.to_owned()
        };
        for c in lc.chars() {
            if c.is_numeric() {
                last = Some(c);
            }
        }

        let d = first.unwrap().to_string() + &last.unwrap().to_string();
        sum += d.parse::<u32>().unwrap();
    }
    sum
}

fn replace_first_spelled_out_digit(l: &str) -> String {
    let mut l = l.to_owned();
    let first_occurence = DIGITS
        .iter()
        .map(|&(s, _)| {
            if let Some(v) = l.find(s) {
                v as i32
            } else {
                i32::MAX
            }
        })
        .collect::<Vec<i32>>(); // NOTE using i32::MAX to indicate that value could not be found :/
    let (digit_idx, idx) = first_occurence
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .unwrap();
    if *idx != i32::MAX {
        l = l.replace(DIGITS[digit_idx].0, DIGITS[digit_idx].1);
    }
    l
}

fn replace_last_spelled_out_digit(l: &str) -> String {
    let mut l = l.to_owned();
    let last_occurence = DIGITS
        .iter()
        .map(|&(s, _)| {
            if let Some(v) = l.rfind(s) {
                v as i32
            } else {
                i32::MIN
            }
        })
        .collect::<Vec<i32>>();
    let (digit_idx, idx) = last_occurence
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap();
    if *idx != i32::MIN {
        l = l.replace(DIGITS[digit_idx].0, DIGITS[digit_idx].1);
    }
    l
}
