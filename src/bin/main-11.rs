use std::fs;

fn main() {
    let filename = "inputs/11.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let image = content
        .lines()
        .map(|l| l.chars().map(Pixel::from).collect::<Vec<Pixel>>())
        .collect::<Vec<Vec<Pixel>>>();
    let empty_rows = determine_empy_rows(&image);
    let empty_cols = determine_empy_cols(&image);
    let galaxy_positions = find_galaxy_positions(&image);

    let sum = galaxy_positions
        .iter()
        .enumerate()
        .map(|(i, &p0)| {
            galaxy_positions
                .iter()
                .skip(i + 1)
                .map(|&p1| compute_distance(p0, p1, &empty_rows, &empty_cols, 1))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("The sum of these lengths is {sum}.");

    let older_sum = galaxy_positions
        .iter()
        .enumerate()
        .map(|(i, &p0)| {
            galaxy_positions
                .iter()
                .skip(i + 1)
                .map(|&p1| compute_distance(p0, p1, &empty_rows, &empty_cols, 1_000_000 - 1))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("The sum of these lengths for the older galaxies is {older_sum}.");
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pixel {
    Galaxy,
    Empty,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Pixel::Galaxy,
            '.' => Pixel::Empty,
            _ => panic!(),
        }
    }
}

fn determine_empy_rows(image: &[Vec<Pixel>]) -> Vec<usize> {
    image
        .iter()
        .enumerate()
        .filter(|(_, l)| l.iter().all(|&p| p == Pixel::Empty))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn determine_empy_cols(image: &[Vec<Pixel>]) -> Vec<usize> {
    let mut empty_cols = Vec::new();
    'outer: for j in 0..image[0].len() {
        for row in image.iter() {
            if row[j] != Pixel::Empty {
                continue 'outer;
            }
        }
        empty_cols.push(j);
    }
    empty_cols
}

fn find_galaxy_positions(image: &[Vec<Pixel>]) -> Vec<Position> {
    let mut positions = Vec::new();
    for (i, row) in image.iter().enumerate() {
        for (j, &p) in row.iter().enumerate() {
            if p == Pixel::Galaxy {
                positions.push(Position { y: i, x: j });
            }
        }
    }
    positions
}

#[derive(Clone, Copy, Debug)]
struct Position {
    y: usize,
    x: usize,
}

fn compute_distance(
    p0: Position,
    p1: Position,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expand_by: usize,
) -> usize {
    let min_y = std::cmp::min(p0.y, p1.y);
    let max_y = std::cmp::max(p0.y, p1.y);
    let expanded_space_y = empty_rows
        .iter()
        .filter(|&&i| i > min_y && i < max_y)
        .count();

    let min_x = std::cmp::min(p0.x, p1.x);
    let max_x = std::cmp::max(p0.x, p1.x);
    let expanded_space_x = empty_cols
        .iter()
        .filter(|&&j| j > min_x && j < max_x)
        .count();

    (max_y - min_y) + (max_x - min_x) + expanded_space_y * expand_by + expanded_space_x * expand_by
}
