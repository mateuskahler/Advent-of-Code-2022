use std::{fs::OpenOptions, io::Read};

const INPUT_PATH: &str = "input.txt";

const ASCII_NEWLINE: u8 = '\n' as u8;
const ASCII_ZERO: u8 = '0' as u8;

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 1:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (u32, u32) {
    let mut input_buffer: Vec<u8> = Vec::new();

    OpenOptions::new()
        .read(true)
        .write(false)
        .open(INPUT_PATH)
        .unwrap()
        .read_to_end(&mut input_buffer)
        .unwrap();

    let values = read_greater_sums(input_buffer.into_iter());

    (values[0], values.iter().sum())
}

fn read_greater_sums(mut input_buffer: impl Iterator<Item = u8>) -> [u32; 3] {
    let mut result = [0; 3];

    let mut sum = 0;
    loop {
        match read_single_number(&mut input_buffer) {
            InputToken::Add(value) => sum += value,
            InputToken::CloseSum => {
                save_if_greater(sum, &mut result);
                sum = 0;
            }
            InputToken::CloseFile => return result,
        }
    }
}

enum InputToken {
    Add(u32),
    CloseSum,
    CloseFile,
}

fn read_single_number(buffer: &mut impl Iterator<Item = u8>) -> InputToken {
    let mut tmp: u32 = match buffer.next() {
        Some(ASCII_NEWLINE) => return InputToken::CloseSum,
        Some(digit) => (digit - ASCII_ZERO) as u32,
        None => return InputToken::CloseFile,
    };

    loop {
        match buffer.next() {
            Some(ASCII_NEWLINE) => return InputToken::Add(tmp),
            Some(digit) => {
                tmp *= 10;
                tmp += (digit - ASCII_ZERO) as u32;
            }
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

fn save_if_greater(value: u32, destination: &mut [u32; 3]) {
    if value > destination[0] {
        destination[2] = destination[1];
        destination[1] = destination[0];
        destination[0] = value;
    } else {
        if value > destination[1] {
            destination[2] = destination[1];
            destination[1] = value;
        } else {
            if value > destination[2] {
                destination[2] = value;
            }
        }
    }
}
