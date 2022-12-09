use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 3:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (u32, u32) {
    let reader = BufReader::new(File::open(INPUT_PATH).unwrap());

    let mut part1: u32 = 0;
    let mut part2 = BagdeGroup::new();

    for input_line in reader.lines() {
        let input_line = input_line.unwrap();
        let input_line = input_line.as_bytes();

        let (stack_a, stack_b) = split_rucksacks(input_line);
        let common_item = find_common_item(stack_a, stack_b);

        part1 += calculate_priority(common_item) as u32;
        part2.add(input_line)
    }

    return (part1, part2.total_priority);
}

fn split_rucksacks<'a>(input_line: &'a [u8]) -> (&'a [u8], &'a [u8]) {
    let mid = input_line.len() / 2;

    return (&input_line[0..mid], &input_line[mid..]);
}

fn find_common_item(stack_a: &[u8], stack_b: &[u8]) -> u8 {
    for item in stack_a {
        if stack_b.contains(item) {
            return *item;
        }
    }

    unreachable!()
}

fn calculate_priority(item: u8) -> u8 {
    if item >= 'a' as u8 {
        return item - 'a' as u8 + 1;
    } else {
        return item - 'A' as u8 + 27;
    }
}

struct BagdeGroup {
    current_line: u32,
    mem: Vec<u8>,
    pub total_priority: u32,
}

impl BagdeGroup {
    pub fn new() -> Self {
        BagdeGroup {
            current_line: 0,
            mem: Vec::with_capacity(64),
            total_priority: 0,
        }
    }

    pub fn add(&mut self, input_line: &[u8]) {
        match self.current_line {
            0 => {
                unsafe {
                    self.mem.set_len(input_line.len());
                }
                self.mem.copy_from_slice(input_line);
                self.current_line = 1;
            }
            1 => {
                self.mem.retain(|e| input_line.contains(e));
                self.current_line = 2
            }
            2 => {
                let item = self.mem.iter().find(|e| input_line.contains(e)).unwrap();
                self.total_priority += calculate_priority(*item) as u32;
                self.current_line = 0;
            }
            _ => unreachable!(),
        };
    }
}
