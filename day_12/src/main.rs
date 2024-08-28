use std::collections::{HashSet, VecDeque};
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
    x: usize,
    y: usize,
}

impl Position {
    fn step(&self, direction: Direction, grid: &[Vec<u32>], is_backwards: bool) -> Option<Self> {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x: usize = (self.x as i32 + dx).try_into().ok()?;
        let new_y: usize = (self.y as i32 + dy).try_into().ok()?;

        let &new_height = grid.get(new_y)?.get(new_x)?;
        let &current_height = grid.get(self.y)?.get(self.x)?;
        let valid = if !is_backwards {
            new_height <= current_height + 1
        } else {
            current_height <= new_height + 1
        };

        if valid {
            Some(Position { x: new_x, y: new_y })
        } else {
            None
        }
    }
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (grid, start, end) = process_input(&input);

    let output_1 = bfs(&grid, start, end, false);
    let output_2 = bfs(&grid, start, end, true);
    println!("part 1: {output_1} part 2: {output_2}");
}

fn process_input(input: &str) -> (Vec<Vec<u32>>, Position, Position) {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(get_height).collect())
        .collect();

    let start = find_char(input, 'S');
    let end = find_char(input, 'E');

    (grid, start, end)
}

fn bfs(grid: &[Vec<u32>], start: Position, end: Position, is_backwards: bool) -> u32 {
    let mut frontier = if !is_backwards {
        VecDeque::from([(start, 0)])
    } else {
        VecDeque::from([(end, 0)])
    };
    let mut visited: HashSet<Position> = HashSet::new();
    while let Some((position, distance)) = frontier.pop_front() {
        if !visited.insert(position) {
            continue;
        }

        if !is_backwards && position == end {
            return distance;
        }

        if grid[position.y][position.x] == 0 && is_backwards {
            return distance;
        }

        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|d| position.step(d, grid, is_backwards))
        .for_each(|next| frontier.push_back((next, distance + 1)));
    }
    println!("{:?}", end);
    panic!("end not reachable")
}

fn get_height(c: char) -> u32 {
    match c {
        'S' => 0,
        'E' => 25,
        c => c as u32 - 'a' as u32,
    }
}

fn find_char(input: &str, c: char) -> Position {
    let (y, x) = input
        .lines()
        .enumerate()
        .find_map(|(i, line)| Some((i, line.chars().position(|x| x == c)?)))
        .unwrap();

    Position { x, y }
}
