use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 9:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (usize, usize) {
    let lines_iter = BufReader::new(File::open(INPUT_PATH).unwrap()).lines();

    let mut head = Pos::new();
    let mut tails = [Pos::new(); 9];

    let mut unique_first_tail: HashSet<String> = HashSet::new();
    let mut unique_last_tail: HashSet<String> = HashSet::new();

    for line in lines_iter {
        let (dir, count) = get_commands(line.unwrap());
        for _ in 0..count {
            head.walk(dir);
            tails[0].follow(&head);

            for tail_index in 1..9 {
                let mut tmp = tails[tail_index].clone();
                tmp.follow(&tails[tail_index - 1]);
                tails[tail_index] = tmp;
            }
            unique_first_tail.insert(tails.first().unwrap().hash_pos());
            unique_last_tail.insert(tails.last().unwrap().hash_pos());
        }
    }

    (unique_first_tail.len(), unique_last_tail.len())
}

#[derive(Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new() -> Self {
        Pos { x: 0, y: 0 }
    }

    pub fn walk(&mut self, dir: char) {
        match dir {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'R' => self.x += 1,
            'L' => self.x -= 1,
            _ => unreachable!(),
        }
    }

    pub fn follow(&mut self, other: &Pos) {
        if self.touches(other) {
            return;
        }

        // this is only possible in part 2 (long rope)
        if self.x.abs_diff(other.x) > 1 && self.y.abs_diff(other.y) > 1 {
            if self.x < other.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
            if self.y < other.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }

        if self.x.abs_diff(other.x) > 1 {
            self.y = other.y;
            if self.x < other.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }

        if self.y.abs_diff(other.y) > 1 {
            self.x = other.x;
            if self.y < other.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }

    pub fn hash_pos(&self) -> String {
        format!("{},{}", self.x, self.y)
    }

    fn touches(&self, other: &Pos) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }
}

fn get_commands(line: String) -> (char, usize) {
    let mut tokens = line.split_ascii_whitespace();
    let c = tokens.next().unwrap().chars().next().unwrap();
    let n = tokens.next().unwrap().parse().unwrap();

    (c, n)
}
