use std::{fmt::Display, fs};

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
    let expanded_image = expand_space(&image, &empty_rows, &empty_cols);
    print_image(&image);
    print_image(&expanded_image);
    let galaxy_positions = find_galaxy_positions(&expanded_image);
    let sum = galaxy_positions
        .iter()
        .map(|&p0| {
            galaxy_positions
                .iter()
                .map(|&p1| compute_distance(p0, p1))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2;
    println!("The sum of these lengths is {sum}.");
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

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Galaxy => write!(f, "#"),
            Pixel::Empty => write!(f, "."),
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

fn expand_space(
    image: &[Vec<Pixel>],
    empty_rows: &[usize],
    empty_cols: &[usize],
) -> Vec<Vec<Pixel>> {
    let mut expanded_image = image.to_vec();
    for &j in empty_cols.iter().rev() {
        for row in expanded_image.iter_mut() {
            row.insert(j, Pixel::Empty);
        }
    }
    for &i in empty_rows.iter().rev() {
        expanded_image.insert(i, vec![Pixel::Empty; expanded_image[0].len()]);
    }
    expanded_image
}

fn print_image(image: &[Vec<Pixel>]) {
    for row in image.iter() {
        for p in row.iter() {
            print!("{p}");
        }
        println!();
    }
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

fn compute_distance(p0: Position, p1: Position) -> usize {
    ((p1.y as i32 - p0.y as i32).abs() + (p1.x as i32 - p0.x as i32).abs()) as usize
}
