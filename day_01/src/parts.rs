use std::cmp::Reverse;

fn process_input(input: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    let mut current = vec![];
    for line in input.lines() {
        if line.is_empty() {
            output.push(current);
            current = vec![];
            continue;
        }
        current.push(line.parse().unwrap())
    }
    if !current.is_empty() {
        output.push(current);
    }
    output
}

pub fn part_1(input: &str) -> i32 {
    let calories = process_input(input);
    calories
        .iter()
        .map(|v| v.iter().sum())
        .max()
        .unwrap()
}

const NUMBER_OF_ELVES: usize = 3;
pub fn part_2(input: &str) -> i32 {
    let mut calories: Vec<i32> = process_input(input)
        .iter()
        .map(|v| v.iter().sum())
        .collect();
    calories.sort_by_key(|&n| Reverse(n));
    calories.iter().take(NUMBER_OF_ELVES).sum()
}
