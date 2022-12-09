use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

const INPUT_PATH: &str = "input.txt";
const ROOT_PATH: &str = "\\";

fn main() {
    let (part1, part2) = solve_problem();

    println!("Day 7:");
    println!("\tpart 1: {}", part1);
    println!("\tpart 2: {}", part2);
}

fn solve_problem() -> (usize, usize) {
    let lines_iter = BufReader::new(File::open(INPUT_PATH).unwrap()).lines();

    let mut path = PathBuf::from(ROOT_PATH);

    let mut file_tree: HashMap<String, Vec<EntryType>> = HashMap::new();
    file_tree.insert(ROOT_PATH.into(), Vec::new());

    for line in lines_iter {
        let line = line.unwrap();

        match parse_line(&line) {
            LineType::Command(command) => match command {
                CommandType::ChandeDir(command) => path = apply_change_dir(path, command),
                CommandType::ListFiles => continue,
            },
            LineType::Stat(stat) => {
                fun_name(stat, &path, &mut file_tree);
            }
        }
    }

    let part1 = sum_dirs_le(&file_tree, 100000);
    let part2 = search_smaller_to(&file_tree);

    (part1, part2)
}

fn fun_name(stat: EntryType, path: &Path, file_tree: &mut HashMap<String, Vec<EntryType>>) {
    if let EntryType::Directory(name) = &stat {
        let new_key: String = path.join(name).to_str().unwrap().into();

        if let Entry::Vacant(v) = file_tree.entry(new_key.clone()) {
            v.insert(Vec::new());
            let key = path.to_str().unwrap();
            let entry = file_tree.get_mut(key).unwrap();
            entry.push(EntryType::Directory(new_key));
        }
    } else {
        let key = path.to_str().unwrap();
        let entry = file_tree.get_mut(key).unwrap();
        entry.push(stat.clone());
    }
}

#[derive(Debug)]
enum LineType {
    Command(CommandType),
    Stat(EntryType),
}

#[derive(Debug)]
enum CommandType {
    ChandeDir(ChangeDirType),
    ListFiles,
}

#[derive(Debug, Clone)]
enum EntryType {
    Directory(String),
    File(String, usize),
}

#[derive(Debug)]
enum ChangeDirType {
    Root,
    Back,
    Into(String),
}

fn parse_line(line: &str) -> LineType {
    let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
    if tokens[0].starts_with('$') {
        match tokens[1] {
            "cd" => match tokens[2] {
                "/" => LineType::Command(CommandType::ChandeDir(ChangeDirType::Root)),
                ".." => LineType::Command(CommandType::ChandeDir(ChangeDirType::Back)),
                _ => LineType::Command(CommandType::ChandeDir(ChangeDirType::Into(
                    tokens[2].into(),
                ))),
            },
            "ls" => LineType::Command(CommandType::ListFiles),
            _ => unreachable!(),
        }
    } else if tokens[0] == "dir" {
        LineType::Stat(EntryType::Directory(tokens[1].into()))
    } else {
        LineType::Stat(EntryType::File(
            tokens[1].into(),
            str::parse(tokens[0]).unwrap(),
        ))
    }
}

fn apply_change_dir(path: PathBuf, command: ChangeDirType) -> PathBuf {
    match command {
        ChangeDirType::Root => PathBuf::from(ROOT_PATH),
        ChangeDirType::Back => path.parent().unwrap().to_path_buf(),
        ChangeDirType::Into(dir) => path.join(dir),
    }
}

fn get_dir_size(dir: &String, file_tree: &HashMap<String, Vec<EntryType>>) -> usize {
    let mut total = 0;
    let entries = file_tree.get(dir).unwrap();

    for entry in entries {
        match entry {
            EntryType::Directory(sub_dir) => {
                total += get_dir_size(sub_dir, file_tree);
            }
            EntryType::File(_, size) => total += size,
        }
    }

    total
}

fn sum_dirs_le(file_tree: &HashMap<String, Vec<EntryType>>, limiar: usize) -> usize {
    let mut total = 0;

    for (dir, _) in file_tree.iter() {
        let dir_size = get_dir_size(dir, file_tree);

        if dir_size <= limiar {
            total += dir_size
        }
    }

    total
}

fn search_smaller_to(file_tree: &HashMap<String, Vec<EntryType>>) -> usize {
    let capacity: i64 = 70000000;
    let desired_free: i64 = 30000000;
    let ocupied = get_dir_size(&"\\".into(), file_tree) as i64;

    let mut dir_to_delete: i64 = 70000000;

    for (dir, _) in file_tree.iter() {
        let dir_size = get_dir_size(dir, file_tree) as i64;
        let result_free = capacity - (ocupied - dir_size);

        if result_free >= desired_free && dir_size < dir_to_delete {
            dir_to_delete = dir_size;
        }
    }

    dir_to_delete as usize
}
