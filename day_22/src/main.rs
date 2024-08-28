use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
}

impl Tile {
    fn new(input: char) -> Self {
        match input {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("invalid character"),
        }
    }
}

#[derive(Clone)]
struct Square(Vec<Vec<Tile>>);

impl Square {
    fn get_tile(&self, position: Position) -> Tile {
        self.0[position.y as usize][position.x as usize]
    }

    fn new(input: &[Vec<char>], x: usize, y: usize, square_size: usize) -> Option<Self> {
        if *input.get(y * square_size)?.get(x * square_size)? == ' ' {
            None
        } else {
            Some(Square(
                (y * square_size..(y + 1) * square_size)
                    .map(|y| {
                        (x * square_size..(x + 1) * square_size)
                            .map(|x| Tile::new(input[y][x]))
                            .collect()
                    })
                    .collect(),
            ))
        }
    }
}

struct Grid {
    faces: Vec<Vec<Option<Square>>>,
    edges: Edges,
}

impl Grid {
    fn new(input: &str, square_size: i32, is_cube: bool) -> Self {
        let square_size = square_size as usize;
        let input = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|s| s.chars().collect_vec())
            .collect_vec();

        let height = input.len() / square_size;
        let width = input.iter().map(|v| v.len()).max().unwrap() / square_size;

        let faces = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Square::new(&input, x, y, square_size))
                    .collect_vec()
            })
            .collect_vec();

        let face_positions = faces
            .iter()
            .map(|v| v.iter().map(|t| t.is_some()).collect_vec())
            .collect_vec();

        let edges = Edges::new(face_positions, is_cube);

        Grid { faces, edges }
    }

    fn get_square(&self, position: Position) -> Square {
        self.faces[position.y as usize][position.x as usize]
            .clone()
            .unwrap()
    }
}

struct Edges(HashMap<(Position, Direction), (Position, Direction)>);

impl Edges {
    fn new(faces: Vec<Vec<bool>>, is_cube: bool) -> Self {
        let height = faces.len();
        let width = faces[0].len();
        let positions = (0..width)
            .cartesian_product(0..height)
            .filter(|&(x, y)| faces[y][x])
            .map(|(x, y)| Position::new(x as i32, y as i32))
            .collect_vec();

        let edges = positions
            .clone()
            .into_iter()
            .flat_map(|p| Direction::all().into_iter().map(move |d| (p, d)))
            .collect_vec();

        if is_cube {
            let mut partitions = Partitions::new(positions);
            partitions.cubify();
            let edges = partitions.get_edge_map();
            Edges(edges)
        } else {
            let edges = edges
                .into_iter()
                .map(|(p, d)| ((p, d), (p.step_wrap_around(d, &faces), d.opposite())))
                .collect();

            Edges(edges)
        }
    }
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
enum DiagonalDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl DiagonalDirection {
    fn get_directions(&self) -> (Direction, Direction) {
        match self {
            DiagonalDirection::DownLeft => (Direction::Down, Direction::Left),
            DiagonalDirection::DownRight => (Direction::Right, Direction::Down),
            DiagonalDirection::UpLeft => (Direction::Left, Direction::Up),
            DiagonalDirection::UpRight => (Direction::Up, Direction::Right),
        }
    }

    fn all() -> [Self; 4] {
        [
            DiagonalDirection::UpLeft,
            DiagonalDirection::UpRight,
            DiagonalDirection::DownLeft,
            DiagonalDirection::DownRight,
        ]
    }
}

struct Partition<T>
where
    T: Eq + Clone + Hash,
{
    data: Vec<HashSet<T>>,
}

impl<T> Partition<T>
where
    T: Eq + Clone + Hash,
{
    fn new() -> Self {
        Partition { data: Vec::new() }
    }

    fn insert(&mut self, item: T) {
        self.data.push(HashSet::from([item]))
    }

    fn get_class(&self, item: T) -> HashSet<T> {
        self.data
            .iter()
            .find(|v| v.contains(&item))
            .unwrap()
            .clone()
    }

    fn merge(&mut self, item_1: T, item_2: T) {
        if self.data.iter().position(|v| v.contains(&item_1)).unwrap()
            == self.data.iter().position(|v| v.contains(&item_2)).unwrap()
        {
            return;
        }

        let i_1 = self.data.iter().position(|v| v.contains(&item_1)).unwrap();
        let class_1 = self.data.remove(i_1);

        let i_2 = self.data.iter().position(|v| v.contains(&item_2)).unwrap();
        let class_2 = self.data.remove(i_2);

        let new_class = class_1.union(&class_2).cloned().collect();
        self.data.push(new_class)
    }
}

impl<T> FromIterator<T> for Partition<T>
where
    T: Eq + Clone + Hash,
{
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        let mut output = Self::new();
        for item in iter {
            output.insert(item);
        }

        output
    }
}

struct Partitions {
    edges: Partition<(Position, Direction)>,
    corners: Partition<(Position, DiagonalDirection)>,
}

impl Partitions {
    fn new(positions: Vec<Position>) -> Self {
        let edges = positions
            .clone()
            .into_iter()
            .flat_map(|p| Direction::all().into_iter().map(move |d| (p, d)))
            .collect_vec();

        let edges = Partition::from_iter(edges);

        let corners = positions
            .into_iter()
            .flat_map(|p| DiagonalDirection::all().into_iter().map(move |d| (p, d)))
            .collect_vec();

        let corners = Partition::from_iter(corners);

        Self { edges, corners }
    }

    fn merge_edges(&mut self, edge_1: (Position, Direction), edge_2: (Position, Direction)) {
        let (corner_1_1, corner_1_2) = (
            (edge_1.0, edge_1.1.get_diagonals().0),
            (edge_1.0, edge_1.1.get_diagonals().1),
        );

        let (corner_2_1, corner_2_2) = (
            (edge_2.0, edge_2.1.get_diagonals().0),
            (edge_2.0, edge_2.1.get_diagonals().1),
        );

        self.edges.merge(edge_1, edge_2);
        self.corners.merge(corner_1_1, corner_2_2);
        self.corners.merge(corner_1_2, corner_2_1);
    }

    fn merge_overlapping_edges(&mut self) {
        let positions = self
            .edges
            .data
            .iter()
            .flatten()
            .map(|&(p, _)| p)
            .collect_vec();

        for position in positions.clone() {
            for direction in Direction::all() {
                let other_position = position.step(direction);
                if positions.contains(&other_position) {
                    self.merge_edges(
                        (position, direction),
                        (other_position, direction.opposite()),
                    )
                }
            }
        }
    }

    fn get_touching_edges(
        &self,
        corners: HashSet<(Position, DiagonalDirection)>,
    ) -> Vec<HashSet<(Position, Direction)>> {
        let edges = corners
            .iter()
            .flat_map(|&(p, d)| {
                let (direction_1, direction_2) = d.get_directions();
                [(p, direction_1), (p, direction_2)]
            })
            .collect_vec();

        let mut output = vec![];
        for edge in edges {
            let class = self.edges.get_class(edge);
            if !output.contains(&class) {
                output.push(class)
            }
        }

        output
    }

    fn get_corner_that_needs_merging(&self) -> Option<HashSet<(Position, DiagonalDirection)>> {
        self.corners
            .data
            .iter()
            .find(|&corners| {
                corners.len() == 3 && self.get_touching_edges(corners.clone()).len() == 4
            })
            .cloned()
    }

    fn merge_corner(&mut self, corner: HashSet<(Position, DiagonalDirection)>) {
        let edges = self.get_touching_edges(corner.clone());
        let singletons = edges.iter().filter(|s| s.len() == 1).collect_vec();
        let edge_1 = *singletons[0].iter().next().unwrap();
        let edge_2 = *singletons[1].iter().next().unwrap();
        self.merge_edges(edge_1, edge_2);
    }

    fn cubify(&mut self) {
        self.merge_overlapping_edges();
        while let Some(c) = self.get_corner_that_needs_merging() {
            self.merge_corner(c);
        }
    }

    fn get_edge_map(&self) -> HashMap<(Position, Direction), (Position, Direction)> {
        self.edges
            .data
            .iter()
            .flat_map(|x| x.iter().permutations(2))
            .map(|v| (*v[0], *v[1]))
            .collect()
    }
}

#[derive(Clone)]
struct Instruction {
    distance: u32,
    rotation: Option<Rotation>,
}

impl Instruction {
    fn new((distance, rotation): (u32, Option<Rotation>)) -> Self {
        Instruction { distance, rotation }
    }
}

#[derive(Clone, Debug)]
enum Rotation {
    Left,
    Right,
}

impl Rotation {
    fn new(input: &str) -> Self {
        match input {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => panic!("invalid input"),
        }
    }
}

struct Instructions(Vec<Instruction>);

impl Instructions {
    fn new(input: &str) -> Self {
        let line = input.lines().last().unwrap();
        let num_pat = Regex::new(r"\d+").unwrap();
        let let_pat = Regex::new(r"[LR]").unwrap();
        let distances: Vec<u32> = num_pat
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let rotations: Vec<Option<Rotation>> = let_pat
            .find_iter(line)
            .map(|m| m.as_str())
            .map(Rotation::new)
            .map(Some)
            .chain([None])
            .collect();

        let instructions: Vec<Instruction> = distances
            .into_iter()
            .zip(rotations)
            .map(Instruction::new)
            .collect();

        Instructions(instructions)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step_wrap_around(&self, direction: Direction, grid: &[Vec<bool>]) -> Self {
        let mut next_square = false;
        let mut current = *self;
        loop {
            if next_square {
                return current;
            }

            current = current
                .step(direction)
                .wrap_around(grid[0].len() as i32, grid.len() as i32);

            next_square = grid[current.y as usize][current.x as usize];
        }
    }

    fn wrap_around(&self, width: i32, height: i32) -> Self {
        Position {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height),
        }
    }

    fn rotate_clockwise(&self, square_size: i32, count: u32) -> Self {
        let mut current = *self;
        for _ in 0..count {
            current = Position {
                y: current.x,
                x: square_size - 1 - current.y,
            };
        }

        current
    }

    fn align(&self, direction: Direction, square_size: i32) -> Self {
        let count = match direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        self.rotate_clockwise(square_size, count)
    }

    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn step(&self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let (new_x, new_y) = (self.x + dx, self.y + dy);

        Position { x: new_x, y: new_y }
    }

    fn is_in_bounds(&self, square_size: i32) -> bool {
        0 <= self.x && self.x < square_size && 0 <= self.y && self.y < square_size
    }

    fn move_to_edge(&self, direction: Direction, square_size: i32) -> Self {
        match direction {
            Direction::Up => Position { y: 0, ..*self },
            Direction::Down => Position {
                y: square_size - 1,
                ..*self
            },

            Direction::Left => Position { x: 0, ..*self },
            Direction::Right => Position {
                x: square_size - 1,
                ..*self
            },
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_diagonals(&self) -> (DiagonalDirection, DiagonalDirection) {
        match self {
            Direction::Up => (DiagonalDirection::UpLeft, DiagonalDirection::UpRight),
            Direction::Down => (DiagonalDirection::DownRight, DiagonalDirection::DownLeft),
            Direction::Left => (DiagonalDirection::DownLeft, DiagonalDirection::UpLeft),
            Direction::Right => (DiagonalDirection::UpRight, DiagonalDirection::DownRight),
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
    fn get_alignment(&self, other: Direction) -> Self {
        let count = match self {
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        };

        other.rotate_clockwise(count)
    }

    fn rotate_clockwise(&self, count: u32) -> Self {
        let mut current = *self;
        for _ in 0..count {
            current = match current {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        }
        current
    }

    fn rotate(&self, rotation: Rotation) -> Self {
        let count = match rotation {
            Rotation::Left => 3,
            Rotation::Right => 1,
        };
        self.rotate_clockwise(count)
    }

    fn opposite(&self) -> Self {
        self.rotate_clockwise(2)
    }
}

#[derive(Clone, Copy, Debug)]
struct Agent {
    position_of_square: Position,
    position_in_square: Position,
    facing: Direction,
}

impl Agent {
    fn new(grid: &Grid) -> Self {
        let y = 0;
        let x = grid.faces[0].iter().position(|x| x.is_some()).unwrap() as i32;
        let facing = Direction::Right;
        let position_of_square = Position::new(x, y);
        let position_in_square = Position::new(0, 0);
        Agent {
            position_of_square,
            position_in_square,
            facing,
        }
    }

    fn apply_instruction(&self, instruction: Instruction, grid: &Grid, square_width: i32) -> Self {
        let mut current = *self;
        for _ in 0..instruction.distance {
            if let Some(next) = current.step(grid, square_width) {
                current = next;
            } else {
                break;
            }
        }

        if let Some(rotation) = instruction.rotation {
            current.facing = current.facing.rotate(rotation);
        }
        current
    }

    fn final_password(&self, square_width: i32) -> u32 {
        let row = self.position_of_square.y * square_width + self.position_in_square.y + 1;
        let column = self.position_of_square.x * square_width + self.position_in_square.x + 1;
        let facing = match self.facing {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        (row * 1000 + column * 4 + facing) as u32
    }

    fn step(&self, grid: &Grid, square_width: i32) -> Option<Self> {
        /*println!(
            "({} {}) ({} {}) {:?}",
            self.position_of_square.x,
            self.position_of_square.y,
            self.position_in_square.x,
            self.position_in_square.y,
            self.facing
        );*/
        let new_position_in_square = self.position_in_square.step(self.facing);
        let new_agent = if new_position_in_square.is_in_bounds(square_width) {
            Agent {
                position_in_square: new_position_in_square,
                ..*self
            }
        } else {
            let new_position_in_square = self
                .position_in_square
                .move_to_edge(self.facing.opposite(), square_width);

            let (new_position_of_square, new_facing) =
                grid.edges.0[&(self.position_of_square, self.facing)];

            let new_facing = new_facing.opposite();
            let alignment = self.facing.get_alignment(new_facing);
            let new_position_in_square = new_position_in_square.align(alignment, square_width);

            Agent {
                position_in_square: new_position_in_square,
                position_of_square: new_position_of_square,
                facing: new_facing,
            }
        };

        let tile = grid
            .get_square(new_agent.position_of_square)
            .get_tile(new_agent.position_in_square);

        if tile == Tile::Wall {
            return None;
        }

        Some(new_agent)
    }
}

fn main() {
    let input = "input";
    let square_size = if input == "input" { 50 } else { 4 };
    let input = read_to_string(input).unwrap();
    let (flat_grid, Instructions(instructions)) = (
        Grid::new(&input, square_size, false),
        Instructions::new(&input),
    );

    let mut agent = Agent::new(&flat_grid);
    for instruction in instructions.clone() {
        agent = agent.apply_instruction(instruction, &flat_grid, square_size);
    }
    let output_1 = agent.final_password(square_size);

    let cube_grid = Grid::new(&input, square_size, true);
    let mut agent = Agent::new(&cube_grid);
    for instruction in instructions {
        agent = agent.apply_instruction(instruction.clone(), &cube_grid, square_size);
    }
    let output_2 = agent.final_password(square_size);

    println!("part 1: {output_1} part 2: {output_2}");
}
