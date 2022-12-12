use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();
    println!("Day 12:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (usize, usize) {
    let mut map = TerrainMap::new();
    let starts = terramorph_start(&mut map.terrain);

    let mut steps_required: Vec<usize> = starts
        .iter()
        .filter_map(|start| map.do_the_walk(start))
        .collect();

    let part1 = steps_required[0];

    steps_required.sort();
    let part2 = steps_required[0];

    (part1, part2)
}

type Position = (i64, i64);

struct TerrainMap {
    pub terrain: Vec<Vec<u8>>,
    pub end: Position,
}

impl TerrainMap {
    pub fn new() -> Self {
        let mut terrain = get_terrain();
        let end = terramorph_end(&mut terrain);

        Self { terrain, end }
    }

    fn do_the_walk(&self, start: &Position) -> Option<usize> {
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(*start);

        let mut pivots = vec![*start];
        for step in 1..100000 {
            let next_try = pivots
                .iter()
                .flat_map(|pivot| {
                    self.paths_from(pivot)
                        .into_iter()
                        .filter(|p| self.value_at(p) <= self.value_at(pivot) + 1)
                        .filter(|p| visited.insert(*p))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if next_try.is_empty() {
                return None;
            }

            if next_try.iter().find(|p| *p == &self.end).is_some() {
                return Some(step);
            }
            pivots = next_try;
        }

        unreachable!()
    }

    fn value_at(&self, pos: &Position) -> u8 {
        self.terrain[pos.1 as usize][pos.0 as usize]
    }

    fn paths_from(&self, pos: &Position) -> Vec<Position> {
        [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ]
        .into_iter()
        .filter(|p| self.position_valid(p))
        .collect()
    }

    fn position_valid(&self, pos: &Position) -> bool {
        (0..self.terrain.len() as i64).contains(&pos.1)
            && (0..self.terrain[0].len() as i64).contains(&pos.0)
    }
}

fn get_terrain() -> Vec<Vec<u8>> {
    BufReader::new(File::open(INPUT_PATH).unwrap())
        .lines()
        .flatten()
        .map(|l| l.into_bytes())
        .collect()
}

fn terramorph_start(terrain: &mut Vec<Vec<u8>>) -> Vec<Position> {
    let mut possible_start = Vec::new();
    for (y, line) in terrain.iter_mut().enumerate() {
        for (x, pixel) in line.iter_mut().enumerate() {
            if *pixel == 'a' as u8 {
                possible_start.push((x as i64, y as i64))
            }
            if *pixel == 'S' as u8 {
                *pixel = 'a' as u8;
                possible_start.insert(0, (x as i64, y as i64))
            }
        }
    }

    possible_start
}

fn terramorph_end(terrain: &mut Vec<Vec<u8>>) -> Position {
    for (y, line) in terrain.iter_mut().enumerate() {
        for (x, pixel) in line.iter_mut().enumerate() {
            if *pixel == 'E' as u8 {
                *pixel = 'z' as u8;
                return (x as i64, y as i64);
            }
        }
    }
    unreachable!()
}
