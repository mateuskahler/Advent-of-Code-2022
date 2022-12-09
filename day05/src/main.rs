use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 5:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (String, String) {
    let mut lines_iter = BufReader::new(File::open(INPUT_PATH).unwrap()).lines();

    let mut boxes1 = Boxes::read_initial_state_from_file(&mut lines_iter);
    let mut boxes2 = boxes1.clone();

    for line in lines_iter {
        let command = extract_command(line.unwrap());
        boxes1.apply_command_single_item_per_move(&command);
        boxes2.apply_command_multiple_items_per_move(&command);
    }

    let part1 = boxes1.items_on_top();
    let part2 = boxes2.items_on_top();

    (part1, part2)
}

#[derive(Clone)]
struct Boxes {
    state: Vec<Vec<u8>>,
}

impl Boxes {
    pub fn read_initial_state_from_file(input: &mut Lines<BufReader<File>>) -> Self {
        let mut number_of_piles: usize = 0;
        let mut state = Vec::new();

        let mut line = input.next().unwrap().unwrap();
        number_of_piles = std::cmp::max(number_of_piles, (line.len() / 4) + 1);
        state.resize(number_of_piles, Vec::new());

        loop {
            for pile_number in 0..number_of_piles {
                let item_index = 1 + pile_number * 4;
                state[pile_number].push(line.as_bytes()[item_index]);
            }

            line = input.next().unwrap().unwrap();
            if line.is_empty() {
                break;
            }
        }

        for pile in &mut state {
            pile.retain(|item| *item >= 'A' as u8 && *item <= 'Z' as u8);
            pile.reverse();
        }

        Boxes { state }
    }

    pub fn apply_command_single_item_per_move(&mut self, command: &PuzzleCommand) {
        for _ in 0..command.move_count {
            let item = self.state[command.source_index].pop().unwrap();
            self.state[command.destination_index].push(item);
        }
    }

    pub fn apply_command_multiple_items_per_move(&mut self, command: &PuzzleCommand) {
        let base_index = self.state[command.source_index].len() - command.move_count;
        let mut items: Vec<u8> = Vec::with_capacity(command.move_count);
        self.state[command.source_index][base_index..].clone_into(&mut items);
        self.state[command.destination_index].extend(items);
        self.state[command.source_index].resize(base_index, 0);
    }

    pub fn items_on_top(&self) -> String {
        String::from_utf8(
            self.state
                .iter()
                .flat_map(|pile| pile.last())
                .map(|item| *item)
                .collect(),
        )
        .unwrap()
    }
}

struct PuzzleCommand {
    pub move_count: usize,
    pub source_index: usize,
    pub destination_index: usize,
}

fn extract_command(input: String) -> PuzzleCommand {
    let values: Vec<usize> = input
        .split_ascii_whitespace()
        .flat_map(|token| token.parse().ok())
        .collect();

    PuzzleCommand {
        move_count: values[0],
        source_index: values[1] - 1,
        destination_index: values[2] - 1,
    }
}
