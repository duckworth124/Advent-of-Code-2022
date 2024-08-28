use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn step(&mut self, direction: Direction) {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        self.x += dx;
        self.y += dy;
    }

    fn pull(&mut self, next: Self) {
        let dx = next.x - self.x;
        let dy = next.y - self.y;

        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        if dx > 0 {
            self.x += 1
        }
        if dx < 0 {
            self.x -= 1
        }
        if dy > 0 {
            self.y += 1
        }
        if dy < 0 {
            self.y -= 1
        }1
    }
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn step(&mut self, direction: Direction) {
        self.knots.get_mut(0).unwrap().step(direction);
        for i in 0..(self.knots.len() - 1) {
            let next = self.knots[i];
            let tail = self.knots.get_mut(i + 1).unwrap();
            tail.pull(next);
        }
    }

    fn new(length: usize) -> Self {
        Rope {
            knots: vec![Position::new(); length],
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let instructions = process_input(&input);

    let output_1 = get_visited(&instructions, 2);
    let output_2 = get_visited(&instructions, 10);

    println!("part 1: {output_1} part 2: {output_2}");
}

fn get_visited(instructions: &Vec<(Direction, u32)>, rope_length: usize) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut rope = Rope::new(rope_length);

    for &(direction, step_count) in instructions {
        for _ in 0..step_count {
            rope.step(direction);
            visited.insert(*rope.knots.last().unwrap());
        }
    }

    visited.len()
}
fn process_input(input: &str) -> Vec<(Direction, u32)> {
    input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line.chars()
                    .skip_while(|&c| c != ' ')
                    .skip(1)
                    .collect::<String>(),
            )
        })
        .map(|(c, s)| {
            let direction = match c {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                c => panic!("invalid character: {c}"),
            };
            let step_count: u32 = s.parse().unwrap();
            (direction, step_count)
        })
        .collect()
}
