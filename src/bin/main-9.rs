use std::fs;

fn main() {
    let filename = "inputs/9.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let histories = content
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum = histories
        .iter()
        .map(|h| *predict(h).last().unwrap())
        .sum::<i32>();
    println!("The sum of these extrapolated values is {sum}.");

    let sum = histories
        .iter()
        .map(|h| {
            let hrev = h.iter().rev().copied().collect::<Vec<i32>>();
            *predict(&hrev).last().unwrap()
        })
        .sum::<i32>();
    println!("The sum of these backward-extrapolated values is {sum}.");
}

fn predict(v: &[i32]) -> Vec<i32> {
    let initial_state = v[0];
    let mut new = step(v);
    new.insert(0, initial_state);
    new.cumsum()
}

fn step(v: &[i32]) -> Vec<i32> {
    let diff = v.to_vec().diff();
    if diff.iter().all(|&v| v == 0) {
        let mut diff = diff;
        diff.push(0);
        diff
    } else {
        let initial_state = diff[0];
        let mut diff = step(&diff);
        diff.insert(0, initial_state);
        diff.cumsum()
    }
}

trait Diff {
    fn diff(&self) -> Vec<i32>;
}

impl Diff for Vec<i32> {
    fn diff(&self) -> Vec<i32> {
        self.windows(2)
            .map(|values| values[1] - values[0])
            .collect::<Vec<i32>>()
    }
}

trait CumSum {
    fn cumsum(&self) -> Vec<i32>;
}

impl CumSum for Vec<i32> {
    fn cumsum(&self) -> Vec<i32> {
        self.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<i32>>()
    }
}
