use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct Cpu {
    x: i32,
}

impl Cpu {
    fn process_instructions(&mut self, instructions: &Vec<Instruction>) -> Vec<i32> {
        let mut x_values = vec![self.x, self.x];
        for &instruction in instructions {
            let &next = x_values.last().unwrap();
            x_values.push(next);
            if let Instruction::AddX(v) = instruction {
                let next = x_values.last().unwrap() + v;
                x_values.push(next);
            }
        }
        x_values
    }

    fn new() -> Self {
        Cpu { x: 1 }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let instructions = process_input(&input);
    let mut cpu = Cpu::new();

    let x_values = cpu.process_instructions(&instructions);

    let output_1 = get_signal_strengths(&x_values);
    let output_2 = render(&get_visible_pixels(&x_values));

    println!("part 1: {output_1}");
    println!("part 2: \n{output_2}");
}

fn get_signal_strengths(x_values: &[i32]) -> i32 {
    let mut output = 0;

    for (value, cycle) in (20..)
        .step_by(40)
        .map(|i| Some((x_values.get(i)?, i)))
        .take_while(|x| x.is_some())
        .flatten()
    {
        let signal_strength = value * cycle as i32;
        output += signal_strength
    }
    output
}
fn process_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::Noop
            } else {
                Instruction::AddX(line.strip_prefix("addx ").unwrap().parse().unwrap())
            }
        })
        .collect()
}

fn get_visible_pixels(x_values: &[i32]) -> Vec<bool> {
    x_values
        .iter()
        .skip(1)
        .enumerate()
        .map(|(i, x)| i.rem_euclid(40).abs_diff(*x as usize) <= 1)
        .collect()
}

const SCREEN_WIDTH: usize = 40;

fn render(visible_pixels: &[bool]) -> String {
    visible_pixels
        .chunks_exact(SCREEN_WIDTH)
        .map(|line| {
            line.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
                + "\n"
        })
        .collect()
}
