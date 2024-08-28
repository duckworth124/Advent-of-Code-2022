use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy)]
enum Direction {
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Instructions(Vec<Direction>);

impl Instructions {
    fn new(input: &str) -> Self {
        Instructions(
            input
                .trim()
                .chars()
                .map(|c| match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    c => panic!("invalid character read: {c}"),
                })
                .collect(),
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Square,
    Horizontal,
    Vertical,
    Plus,
    BackwardsL,
}

impl Shape {
    fn get_relative_positions(self) -> Vec<Position> {
        let positions = match self {
            Shape::BackwardsL => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Shape::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Shape::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Shape::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            Shape::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        };

        positions
            .iter()
            .map(|(x, y)| Position { x: *x, y: *y })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct Block {
    shape: Shape,
    position: Position,
}

impl Block {
    fn new(shape: Shape, state: &State) -> Self {
        let position = Position::new(state);
        Block { position, shape }
    }

    fn step(&self, instruction: Direction, state: &State) -> Option<Self> {
        let new_block = Block {
            position: self.position.step(instruction),
            ..*self
        };

        if state.is_blocked(new_block) {
            None
        } else {
            Some(new_block)
        }
    }

    fn get_occupied_points(&self) -> Vec<Position> {
        let relative_positions = self.shape.get_relative_positions();
        relative_positions
            .iter()
            .map(|p| p.add(self.position))
            .collect()
    }
}

struct State {
    blocked_positions: HashSet<Position>,
    instructions: Instructions,
    instruction_index: usize,
    floor_height: i64,
}

impl State {
    fn new(instructions: Instructions) -> Self {
        State {
            blocked_positions: HashSet::new(),
            instructions,
            instruction_index: 0,
            floor_height: 0,
        }
    }

    fn next_instruction(&mut self) -> Direction {
        let output = self.instructions.0[self.instruction_index];
        self.instruction_index += 1;
        self.instruction_index %= self.instructions.0.len();
        output
    }

    fn drop_block(&mut self, shape: Shape) {
        let mut block = Block::new(shape, self);
        loop {
            if let Some(new_block) = block.step(self.next_instruction(), self) {
                block = new_block;
            }
            if let Some(new_block) = block.step(Direction::Down, self) {
                block = new_block
            } else {
                break;
            }
        }
        for p in block.get_occupied_points() {
            self.blocked_positions.insert(p);
        }

        let min_height = (0..=self.max_height())
            .find(|y| (0..=6).all(|x| self.blocked_positions.contains(&Position { x, y: *y })))
            .unwrap_or(0);

        if min_height > 0 {
            self.floor_height += min_height;
            self.blocked_positions = self
                .blocked_positions
                .iter()
                .map(|p| Position {
                    y: p.y - min_height,
                    x: p.x,
                })
                .filter(|p| p.y > 0)
                .collect();
        }
    }

    fn max_height(&self) -> i64 {
        self.blocked_positions
            .iter()
            .map(|p| p.y)
            .max()
            .unwrap_or(0)
    }

    fn is_blocked(&self, block: Block) -> bool {
        let occupied_points = block.get_occupied_points();
        occupied_points
            .iter()
            .any(|p| self.blocked_positions.contains(p) || p.x < 0 || p.x >= 7 || p.y == 0)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(state: &State) -> Self {
        let max_height = state.max_height();
        Position {
            x: 2,
            y: max_height + 4,
        }
    }

    fn step(&self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (new_x, new_y) = (self.x + dx, self.y + dy);
        Position { x: new_x, y: new_y }
    }

    fn add(self, other: Self) -> Self {
        let (x, y) = (self.x + other.x, self.y + other.y);
        Position { x, y }
    }
}

fn simulate_blocks(instructions: Instructions, mut number_of_blocks: u64) -> i64 {
    let mut state = State::new(instructions);
    let shapes = [
        Shape::Horizontal,
        Shape::Plus,
        Shape::BackwardsL,
        Shape::Vertical,
        Shape::Square,
    ];

    let mut previous_states = vec![];
    for shape in shapes.iter().cycle() {
        if let Some(cycle_start) =
            previous_states
                .iter()
                .position(|((blocked_points, instruction_index, s), _)| {
                    &state.blocked_positions == blocked_points
                        && &state.instruction_index == instruction_index
                        && &shape == s
                })
        {
            println!("cycle found");

            let cycle_length = (previous_states.len() - cycle_start) as u64;
            let number_of_cycles = number_of_blocks / cycle_length;
            let height_change = state.floor_height - previous_states[cycle_start].1;
            state.floor_height += height_change * number_of_cycles as i64;
            number_of_blocks -= cycle_length * number_of_cycles;
            previous_states.clear();
        }

        previous_states.push((
            (
                state.blocked_positions.clone(),
                state.instruction_index,
                shape,
            ),
            state.floor_height,
        ));

        state.drop_block(*shape);
        number_of_blocks -= 1;
        if number_of_blocks == 0 {
            break;
        }
    }

    state.max_height() + state.floor_height
}

fn main() {
    let input = read_to_string("input").unwrap();
    let instructions = Instructions::new(&input);

    let output_1 = simulate_blocks(instructions.clone(), 2022);
    let output_2 = simulate_blocks(instructions, 1000000000000);

    println!("part 1: {output_1} part 2: {output_2}");
}
