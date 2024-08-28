use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    let input = process_input(&input);

    println!(
        "part 1: {} part 2: {}",
        part_1(input.clone()),
        part_2(input)
    );
}

fn part_1((mut stacks, instructions): (Vec<Vec<char>>, Vec<(u32, usize, usize)>)) -> String {
    for (count, from, to) in instructions {
        move_crates(&mut stacks, count, from, to);
    }

    let mut output = "".to_string();
    for stack in stacks {
        output.push(stack.last().unwrap().clone());
    }

    output
}

fn part_2((mut stacks, instructions): (Vec<Vec<char>>, Vec<(u32, usize, usize)>)) -> String {
    for (count, from, to) in instructions {
        move_multiple_crates(&mut stacks, count, from, to);
    }

    let mut output = "".to_string();
    for stack in stacks {
        output.push(stack.last().unwrap().clone());
    }

    output
}
fn move_crates(stacks: &mut Vec<Vec<char>>, count: u32, from: usize, to: usize) {
    let from = from - 1;
    let to = to - 1;
    for _ in 0..count {
        let current_crate = stacks[from].pop().unwrap();
        stacks[to].push(current_crate);
    }
}

fn move_multiple_crates(stacks: &mut Vec<Vec<char>>, count: u32, from: usize, to: usize) {
    let from = from - 1;
    let to = to - 1;
    let mut crates = vec![];
    for _ in 0..count {
        crates.push(stacks[from].pop().unwrap())
    }

    for _ in 0..count {
        stacks[to].push(crates.pop().unwrap());
    }
}

fn process_input(input: &str) -> (Vec<Vec<char>>, Vec<(u32, usize, usize)>) {
    let crates: Vec<_> = input
        .lines()
        .take_while(|l| l.chars().nth(1).unwrap() != '1')
        .collect();

    let crates: Vec<_> = crates
        .into_iter()
        .rev()
        .map(|line| line.chars().skip(1).step_by(4).collect::<String>())
        .collect();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; crates.first().unwrap().len()];
    for row in crates {
        for (index, current_crate) in row.chars().enumerate() {
            if current_crate != ' ' {
                stacks[index].push(current_crate);
            }
        }
    }

    let instructions: Vec<&str> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();

    let instructions = instructions
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter(|s| s.chars().next().unwrap().is_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|v| {
            let count = v[0].parse().unwrap();
            let from = v[1].parse().unwrap();
            let to = v[2].parse().unwrap();
            (count, from, to)
        })
        .collect();

    (stacks, instructions)
}
