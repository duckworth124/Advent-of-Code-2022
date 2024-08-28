use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = read_to_string("input").unwrap();
    let grid = process_input(&input);

    let output_1 = count_visible(&grid);
    let output_2 = max_scenic_score(&grid);

    println!("part 1: {output_1} part 2: {output_2}");
}

fn is_visible_from_direction(
    grid: &Vec<Vec<u32>>,
    (x, y): (usize, usize),
    direction: Direction,
) -> bool {
    let values_to_check: Vec<u32> = match direction {
        Direction::Up => (0..y).map(|i| grid[i][x]).collect(),
        Direction::Down => (y + 1..grid.len()).map(|i| grid[i][x]).collect(),
        Direction::Left => (0..x).map(|i| grid[y][i]).collect(),
        Direction::Right => (x + 1..grid[0].len()).map(|i| grid[y][i]).collect(),
    };

    values_to_check.iter().all(|&n| n < grid[y][x])
}

fn is_visible(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .any(|&d| is_visible_from_direction(&grid, pos, d))
}

fn count_visible(grid: &Vec<Vec<u32>>) -> usize {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&pos| is_visible(grid, pos))
        .count()
}

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn viewing_distance(grid: &Vec<Vec<u32>>, (x, y): (usize, usize), direction: Direction) -> usize {
    let values_to_check: Vec<u32> = match direction {
        Direction::Up => (0..y).map(|i| grid[i][x]).rev().collect(),
        Direction::Down => (y + 1..grid.len()).map(|i| grid[i][x]).collect(),
        Direction::Left => (0..x).map(|i| grid[y][i]).rev().collect(),
        Direction::Right => (x + 1..grid[0].len()).map(|i| grid[y][i]).collect(),
    };

    if values_to_check.len() == 0 {
        return 0;
    };

    values_to_check
        .into_iter()
        .scan(true, |visible, n| {
            if !*visible {
                return Some((n, false));
            };
            if n >= grid[y][x] {
                *visible = false
            };
            Some((n, true))
        })
        .take_while(|&(_, b)| b)
        .count()
}

fn scenic_score(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> usize {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .map(|d| viewing_distance(grid, pos, d))
    .product()
}

fn max_scenic_score(grid: &Vec<Vec<u32>>) -> usize {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|pos| scenic_score(grid, pos))
        .max()
        .unwrap()
}
