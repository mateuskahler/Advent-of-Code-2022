use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 8:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (usize, usize) {
    let lines_iter = BufReader::new(File::open(INPUT_PATH).unwrap()).lines();

    let forest = lines_iter
        .map(|l| l.unwrap())
        .map(|l| l.as_bytes().iter().map(|t| *t - '0' as u8).collect())
        .collect();

    let part1 = count_visible_trees(&forest);
    let part2 = get_best_tree(forest);

    (part1, part2)
}

fn count_visible_trees(forest: &Vec<Vec<u8>>) -> usize {
    let horizontal = horizontal_visibility(&forest);
    let vertical = horizontal_visibility(&transpose(&forest));
    let mut visible_trees_count = 0;

    for y in 0..horizontal.len() {
        for x in 0..vertical.len() {
            if horizontal[y][x] > 0 || vertical[x][y] > 0 {
                visible_trees_count += 1;
            }
        }
    }
    visible_trees_count
}

fn get_best_tree(forest: Vec<Vec<u8>>) -> usize {
    let mut best_score = 0;

    for y in 0..forest.len() {
        for x in 0..forest[0].len() {
            best_score = std::cmp::max(best_score, tree_score(&forest, x, y))
        }
    }
    best_score
}

fn horizontal_visibility(m: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let c_max = m[0].len();
    let l_max = m.len();

    let mut vc = vec![vec![0; c_max]; l_max];

    for li in 0..l_max {
        //from the left
        vc[li][0] = 1;
        let mut t = m[li][0];
        for ci in 1..c_max - 1 {
            if m[li][ci] > t {
                vc[li][ci] = 1;
                t = m[li][ci];
            }
        }

        //from the right
        vc[li][c_max - 1] = 1;
        t = m[li][c_max - 1];
        for ci in (1..c_max - 1).rev() {
            if m[li][ci] > t {
                vc[li][ci] = 1;
                t = m[li][ci];
            }
        }
    }

    vc
}

fn tree_score(forest: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let c_max = forest[0].len();
    let l_max = forest.len();
    let threshold = forest[y][x];

    let mut visible_trees = [0; 4];

    // to the right
    for scan_x in (x + 1)..c_max {
        visible_trees[0] += 1;
        if forest[y][scan_x] >= threshold {
            break;
        }
    }

    // to the left
    for scan_x in (0..x).rev() {
        visible_trees[1] += 1;
        if forest[y][scan_x] >= threshold {
            break;
        }
    }

    // up
    for scan_y in (y + 1)..l_max {
        visible_trees[2] += 1;
        if forest[scan_y][x] >= threshold {
            break;
        }
    }

    // down
    for scan_y in (0..y).rev() {
        visible_trees[3] += 1;
        if forest[scan_y][x] >= threshold {
            break;
        }
    }

    visible_trees.iter().product()
}

fn transpose(m: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let lines = m.len();
    let columns = m[0].len();

    (0..columns)
        .map(|c| (0..lines).map(|l| m[l][c]).collect())
        .collect()
}
