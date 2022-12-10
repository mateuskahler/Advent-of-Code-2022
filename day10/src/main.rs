use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    println!("Day 10:");
    println!("\tpart 1: {}", part1());
    println!("\tpart 2:");
    part2();
}

fn part1() -> i64 {
    let mut watchdog_clock: usize = 20;
    let mut register_x = 1;
    let mut accum: i64 = 0;

    for (clock, opcode) in get_commands().iter().enumerate() {
        if let Ok(value) = opcode.parse::<i64>() {
            register_x += value;
        }

        if (clock + 1) == watchdog_clock {
            accum += (clock + 1) as i64 * register_x;
            watchdog_clock += 40;
        }
    }

    accum
}

fn part2() {
    let mut screen = ['.' as u8; 240];
    let mut register_x = 1;

    for (clock, opcode) in get_commands().iter().enumerate() {
        emit_pixel(register_x, clock, &mut screen);
        if let Ok(value) = opcode.parse::<i64>() {
            register_x += value;
        }
    }

    print_screen(screen)
}

fn emit_pixel(register_x: i64, clock: usize, screen: &mut [u8; 240]) {
    let sprite_pos = (register_x - 1)..=(register_x + 1);
    if sprite_pos.contains(&((clock % 40) as i64)) {
        screen[clock] = '#' as u8;
    }
}

fn print_screen(screen: [u8; 240]) {
    for (n, pixel) in screen.iter().enumerate() {
        if n % 40 == 0 {
            println!("")
        }
        print!("{}", *pixel as char)
    }
}

fn get_commands() -> Vec<String> {
    BufReader::new(File::open(INPUT_PATH).unwrap())
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .collect()
}
