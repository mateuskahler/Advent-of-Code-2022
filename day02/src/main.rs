use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
};
const INPUT_PATH: &str = "input.txt";

// Reading each line as an unsigned 32bit integer
const POSSIBLE_ENTRY_0: u32 = 0x0a582041; //  "A X\n" Part 1: Rock Rock -> Draw;        Part 2: Rock Lose -> Scissor
const POSSIBLE_ENTRY_1: u32 = 0x0a582042; //  "B X\n" Part 1: Paper Rock -> Lose;       Part 2: Paper Lose -> Rock
const POSSIBLE_ENTRY_2: u32 = 0x0a582043; //  "C X\n" Part 1: Scissor Rock -> Win;      Part 2: Scissor Lose -> Paper
const POSSIBLE_ENTRY_3: u32 = 0x0a592041; //  "A Y\n" Part 1: Rock Paper -> Win;        Part 2: Rock Draw -> Rock
const POSSIBLE_ENTRY_4: u32 = 0x0a592042; //  "B Y\n" Part 1: Paper Paper -> Draw;      Part 2: Paper Draw -> Paper
const POSSIBLE_ENTRY_5: u32 = 0x0a592043; //  "C Y\n" Part 1: Scissor Paper -> Lose;    Part 2: Scissor Draw -> Scissor
const POSSIBLE_ENTRY_6: u32 = 0x0a5a2041; //  "A Z\n" Part 1: Rock Scissor -> Lose;     Part 2: Rock Win -> Paper
const POSSIBLE_ENTRY_7: u32 = 0x0a5a2042; //  "B Z\n" Part 1: Paper Scissor -> Win;     Part 2: Paper Win -> Scissor
                                          //  "C Z\n" Part 1: Scissor Scissor -> Draw;  Part 2: Scissor Win -> Rock (implicit 0x0a5a2043)

const ROCK_POINTS: u32 = 1;
const PAPER_POINTS: u32 = 2;
const SCISSOR_POINTS: u32 = 3;

const WIN_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;
const LOSE_POINTS: u32 = 0;

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 2:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (u32, u32) {
    let mut reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .write(false)
            .open(INPUT_PATH)
            .unwrap(),
    );
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    let mut buf = [0; 4];
    while reader.read_exact(&mut buf).is_ok() {
        let entry = u32::from_le_bytes(buf);

        part1 += points_from_defined_selections(entry);
        part2 += points_from_defined_outcome(entry);
    }

    (part1, part2)
}

fn points_from_defined_selections(entry: u32) -> u32 {
    match entry {
        POSSIBLE_ENTRY_0 => return ROCK_POINTS + DRAW_POINTS,
        POSSIBLE_ENTRY_1 => return ROCK_POINTS + LOSE_POINTS,
        POSSIBLE_ENTRY_2 => return ROCK_POINTS + WIN_POINTS,
        POSSIBLE_ENTRY_3 => return PAPER_POINTS + WIN_POINTS,
        POSSIBLE_ENTRY_4 => return PAPER_POINTS + DRAW_POINTS,
        POSSIBLE_ENTRY_5 => return PAPER_POINTS + LOSE_POINTS,
        POSSIBLE_ENTRY_6 => return SCISSOR_POINTS + LOSE_POINTS,
        POSSIBLE_ENTRY_7 => return SCISSOR_POINTS + WIN_POINTS,
        _ => return SCISSOR_POINTS + DRAW_POINTS,
    }
}

fn points_from_defined_outcome(entry: u32) -> u32 {
    match entry {
        POSSIBLE_ENTRY_0 => return SCISSOR_POINTS + LOSE_POINTS,
        POSSIBLE_ENTRY_1 => return ROCK_POINTS + LOSE_POINTS,
        POSSIBLE_ENTRY_2 => return PAPER_POINTS + LOSE_POINTS,
        POSSIBLE_ENTRY_3 => return ROCK_POINTS + DRAW_POINTS,
        POSSIBLE_ENTRY_4 => return PAPER_POINTS + DRAW_POINTS,
        POSSIBLE_ENTRY_5 => return SCISSOR_POINTS + DRAW_POINTS,
        POSSIBLE_ENTRY_6 => return PAPER_POINTS + WIN_POINTS,
        POSSIBLE_ENTRY_7 => return SCISSOR_POINTS + WIN_POINTS,
        _ => return ROCK_POINTS + WIN_POINTS,
    }
}
