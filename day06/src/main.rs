use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 6:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (usize, usize) {
    let line = BufReader::new(File::open(INPUT_PATH).unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let part1 = how_many_until_n_different(&line, 4);
    let part2 = how_many_until_n_different(&line, 14);

    (part1, part2)
}

fn all_differ(input: &[u8]) -> bool {
    for (i, v) in input.iter().enumerate() {
        if input[i + 1..].contains(v) {
            return false;
        }
    }
    true
}

fn how_many_until_n_different(input: &String, n: usize) -> usize {
    for (i, v) in input.as_bytes().iter().as_slice().windows(n).enumerate() {
        if all_differ(v) {
            return i + n;
        }
    }
    unreachable!()
}
