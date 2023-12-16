use std::fs;

fn main() {
    let filename = "inputs/6.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let content = content.lines().collect::<Vec<&str>>();
    let time = extract_numbers_after_colon(content[0]);
    let record_distance = extract_numbers_after_colon(content[1]);
    let possible_distances = time
        .iter()
        .map(compute_distances)
        .collect::<Vec<Vec<usize>>>();
    let nways = possible_distances
        .iter()
        .zip(record_distance.iter())
        .map(|(pd, &rd)| pd.iter().filter(|&&d| d > rd).count())
        .collect::<Vec<usize>>();
    println!(
        "When you multiply the number of ways you can beat the record together you get {:?}.",
        nways.iter().product::<usize>()
    );
}

fn extract_numbers_after_colon(s: &str) -> Vec<usize> {
    s.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn compute_distances(t: &usize) -> Vec<usize> {
    (0..*t + 1)
        .into_iter()
        .map(|ti| (*t - ti) * ti)
        .collect::<Vec<usize>>()
}
