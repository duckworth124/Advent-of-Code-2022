use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = get_marker_position(&input, 4);
    let output_2 = get_marker_position(&input, 14);
    println!("part 1: {output_1} part 2: {output_2}");
}

fn is_unique(input: &[char]) -> bool {
    let mut seen = HashSet::new();
    input.iter().all(|&c| seen.insert(c))
}

fn get_marker_position(input: &str, length: usize) -> usize {
    input.chars()
        .collect::<Vec<char>>()
        .windows(length)
        .position(is_unique)
        .unwrap()
    + length
}