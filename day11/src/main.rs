use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    println!("Day 11:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2: {}", part2());
}

fn part1() -> usize {
    let lines: Vec<String> = BufReader::new(File::open(INPUT_PATH).unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    let mut monkeys = Monkey::from_lines(lines, false);

    let rounds = 20;

    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            let activities = monkeys[m].inspect_all();
            for (value, destination) in activities {
                monkeys[destination].items.push_back(value);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    inspects.sort_by(|a, b| b.cmp(a));

    inspects[0] * inspects[1]
}

fn part2() -> usize {
    let lines: Vec<String> = BufReader::new(File::open(INPUT_PATH).unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    let mut monkeys = Monkey::from_lines(lines, true);

    let rounds = 10000;

    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            let activities = monkeys[m].inspect_all();
            for (value, destination) in activities {
                monkeys[destination].items.push_back(value);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    inspects.sort_by(|a, b| b.cmp(a));

    inspects[0] * inspects[1]
}

#[derive(Debug)]
enum InspectOp {
    Mul(usize),
    Sum(usize),
    Square,
}

impl InspectOp {
    pub fn apply(&self, old: usize) -> usize {
        match self {
            InspectOp::Mul(value) => old * value,
            InspectOp::Sum(value) => old + value,
            InspectOp::Square => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<usize>,
    pub inspect: InspectOp,
    pub divisible_test_value: usize,
    pub target_monkey_true: usize,
    pub target_monkey_false: usize,
    pub inspect_count: usize,
    pub modulate_worries: bool,
}

impl Monkey {
    pub fn from_lines(lines: Vec<String>, modulate_worries: bool) -> Vec<Self> {
        let mut offset = 0;
        let mut monkeys = Vec::new();

        while lines.len() > offset + 2 {
            let items = VecDeque::from(get_numbers(&lines[offset + 1], ','));
            let inspect = match get_number(&lines[offset + 2]) {
                Some(value) => {
                    if lines[offset + 2].contains('+') {
                        InspectOp::Sum(value)
                    } else {
                        InspectOp::Mul(value)
                    }
                }
                None => InspectOp::Square,
            };
            let divisible_test_value = get_number(&lines[offset + 3]).unwrap();
            let target_monkey_true = get_number(&lines[offset + 4]).unwrap();
            let target_monkey_false = get_number(&lines[offset + 5]).unwrap();

            monkeys.push(Self {
                items,
                inspect,
                divisible_test_value,
                target_monkey_true,
                target_monkey_false,
                inspect_count: 0,
                modulate_worries,
            });

            offset += 7;
        }

        monkeys
    }

    pub fn inspect_all(&mut self) -> Vec<(usize, usize)> {
        self.inspect_count += self.items.len();
        let result = self.items.iter().map(|v| self.inspect(*v)).collect();
        self.items.clear();

        result
    }

    fn inspect(&self, value: usize) -> (usize, usize) {
        let new = if self.modulate_worries {
            self.inspect.apply(value) % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23)
        } else {
            self.inspect.apply(value) / 3
        };

        if new % self.divisible_test_value == 0 {
            (new, self.target_monkey_true)
        } else {
            (new, self.target_monkey_false)
        }
    }
}

fn get_number(string: &str) -> Option<usize> {
    let mut string = string.to_owned();
    string.retain(|c| c.is_ascii_digit());
    string.parse().ok()
}

fn get_numbers(string: &str, delim: char) -> Vec<usize> {
    let mut string = string.to_owned();
    string.retain(|c| c.is_numeric() || c == delim);
    string.split(delim).map(|s| s.parse().unwrap()).collect()
}
