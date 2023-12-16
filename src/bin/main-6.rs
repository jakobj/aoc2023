use std::fs;

fn main() {
    let filename = "inputs/6.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let content = content.lines().collect::<Vec<&str>>();

    // part 1
    let time = extract_numbers_after_colon(content[0]);
    let record_distance = extract_numbers_after_colon(content[1]);
    let possible_distances = time
        .into_iter()
        .map(compute_distances)
        .collect::<Vec<Vec<usize>>>();
    let nways = possible_distances
        .into_iter()
        .zip(record_distance)
        .map(|(pd, rd)| pd.iter().filter(|&&d| d > rd).count())
        .collect::<Vec<usize>>();
    println!(
        "When you multiply the number of ways you can beat the record together you get {:?}.",
        nways.iter().product::<usize>()
    );

    // brute force part 2
    let time = extract_single_number_after_colon(content[0]);
    let record_distance = extract_single_number_after_colon(content[1]);
    let possible_distances = compute_distances(time);
    let nways = possible_distances
        .into_iter()
        .filter(|&pd| pd > record_distance)
        .count();
    println!(
        "There are {nways} ways in which you can beat the record in this one much longer race."
    );
}

fn extract_numbers_after_colon(s: &str) -> Vec<usize> {
    s.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn compute_distances(t: usize) -> Vec<usize> {
    (0..t + 1).map(|ti| (t - ti) * ti).collect::<Vec<usize>>()
}

fn extract_single_number_after_colon(s: &str) -> usize {
    s.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}
