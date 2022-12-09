use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 4:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (u32, u32) {
    let reader = BufReader::new(File::open(INPUT_PATH).unwrap());

    let mut part1 = 0;
    let mut part2 = 0;

    reader
        .lines()
        .map(|line| extract_ranges(line.unwrap()))
        .for_each(|ranges| {
            if one_range_fully_contains_other(&ranges) {
                part1 += 1;
            }
            if one_range_partially_contains_other(&ranges) {
                part2 += 1;
            }
        });

    (part1, part2)
}

fn extract_ranges(input_line: String) -> [RangeInclusive<u32>; 2] {
    let extract_range = |range_text: &str| {
        let mut limits = range_text.split('-');
        let start = limits.next().unwrap();
        let end = limits.next().unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    };

    let mut ranges = input_line.split(',');
    let range_a = extract_range(ranges.next().unwrap());
    let range_b = extract_range(ranges.next().unwrap());

    [range_a, range_b]
}

fn one_range_fully_contains_other(ranges: &[RangeInclusive<u32>; 2]) -> bool {
    (ranges[0].contains(ranges[1].start()) && ranges[0].contains(ranges[1].end()))
        || (ranges[1].contains(ranges[0].start()) && ranges[1].contains(ranges[0].end()))
}

fn one_range_partially_contains_other(ranges: &[RangeInclusive<u32>; 2]) -> bool {
    ranges[0].contains(ranges[1].start())
        || ranges[0].contains(ranges[1].end())
        || ranges[1].contains(ranges[0].start())
        || ranges[1].contains(ranges[0].end())
}
