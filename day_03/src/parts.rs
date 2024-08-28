fn get_common_item(line: &str) -> char {
    let (left, right) = line.split_at(line.chars().count() / 2);
    left.chars()
        .find(|&c| right.contains(c))
        .unwrap()
}

fn get_priority(c: char) -> u32 {
    let mut output = 0;
    if c.is_uppercase() {
        output += 26;
    };
    output += c.to_ascii_lowercase() as u32 - 'a' as u32 + 1;
    output
}

pub fn part_1(input: &str) -> u32 {
    input.lines()
        .map(|line| get_priority(get_common_item(line)))
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|trio| trio[0].chars().find(|&c| trio[1].contains(c) && trio[2].contains(c)).unwrap())
        .map(get_priority)
        .sum()
}