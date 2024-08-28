use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

struct Directory {
    directories: HashSet<String>,
    files: HashMap<String, u32>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            directories: HashSet::new(),
            files: HashMap::new(),
        }
    }
}

const MAX_FILE_SIZE: u32 = 100000;
const TOTAL_DISK_SPACE: u32 = 70000000;
const REQUIRED_SPACE: u32 = 30000000;
fn main() {
    let input = read_to_string("input").unwrap();
    let directories = process_input(&input);

    let output_1 = part_1(&directories);
    let output_2 = part_2(&directories);

    println!("part 1: {output_1} part 2: {output_2}");
}

fn part_1(directories: &HashMap<Vec<String>, Directory>) -> u32 {
    let mut output = 0;
    for path in directories.keys() {
        let size = get_size(&directories, path.clone());
        if size <= MAX_FILE_SIZE {
            output += size
        }
    }

    output
}

fn part_2(directories: &HashMap<Vec<String>, Directory>) -> u32 {
    let min_file_size = REQUIRED_SPACE + get_size(directories, vec![]) - TOTAL_DISK_SPACE;
    directories
        .keys()
        .map(|path| get_size(directories, path.clone()))
        .filter(|&n| n >= min_file_size)
        .min()
        .unwrap()
}

fn get_size(directories: &HashMap<Vec<String>, Directory>, path: Vec<String>) -> u32 {
    let mut size = 0;
    let directory = &directories[&path];
    size += directory.files.values().sum::<u32>();

    for sub_directory in directory.directories.clone() {
        let new_path = [path.clone(), vec![sub_directory]].concat();
        size += get_size(directories, new_path);
    }

    size
}

fn process_input(input: &str) -> HashMap<Vec<String>, Directory> {
    let mut directories: HashMap<Vec<String>, Directory> = HashMap::new();
    directories.insert(vec![], Directory::new());
    let mut current_path = vec![];

    for line in input.lines() {
        if line == "$ ls" {
            continue;
        }
        if line.starts_with("$ cd") {
            let next_directory = line.split_whitespace().last().unwrap();
            match next_directory {
                ".." => {
                    current_path.pop();
                }

                "/" => {
                    current_path.clear();
                }

                next => {
                    current_path.push(next.to_string());
                    directories
                        .entry(current_path.clone())
                        .or_insert(Directory::new());
                }
            };
            continue;
        }

        if line.starts_with("dir") {
            let next = line.chars().skip(4).collect();
            directories
                .get_mut(&current_path)
                .unwrap()
                .directories
                .insert(next);
            continue;
        }

        let file_size = line
            .chars()
            .take_while(|&c| c != ' ')
            .collect::<String>()
            .parse()
            .unwrap();

        let file_name = line.chars().skip_while(|&c| c != ' ').skip(1).collect();

        directories
            .get_mut(&current_path)
            .unwrap()
            .files
            .insert(file_name, file_size);
    }

    directories
}
