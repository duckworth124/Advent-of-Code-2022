use regex::Regex;

fn does_one_contain_other((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> bool {
    start1 <= start2 && end2 <= end1 ||
        start2 <= start1 && end1 <= end2
}

fn is_overlap((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> bool {
    start1 <= start2 && start2 <= end1 ||
        start2 <= start1 && start1 <= end2
}

fn process_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let number_re = Regex::new(r"\d+").unwrap();
    let numbers: Vec<u32> = number_re.find_iter(line).map(|m| m.as_str().parse().unwrap()).collect();
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

fn process_input(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input.lines()
        .map(process_line)
        .collect()
}

pub fn part_1(input: &str) -> usize {
    process_input(input)
        .iter()
        .filter(|(a, b)| does_one_contain_other(*a, *b))
        .count()
}

pub fn part_2(input: &str) -> usize {
    process_input(input)
        .iter()
        .filter(|(a, b)| is_overlap(*a, *b))
        .count()
}