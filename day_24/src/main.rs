use itertools::Itertools;
use num::integer::lcm;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet, fs::read_to_string};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: char) -> Option<Self> {
        match input {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn manhattan_distance(&self, other: Position) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn step(&self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (x, y) = (self.x + dx, self.y + dy);
        Position { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

impl Blizzard {
    fn new(position: Position, direction: Direction) -> Self {
        Blizzard {
            position,
            direction,
        }
    }

    fn step(&self, max_x: i32, max_y: i32) -> Self {
        let mut position = self.position.step(self.direction);
        if position.x == 0 {
            position.x = max_x;
        } else if position.x > max_x {
            position.x = 1;
        };

        if position.y == 0 {
            position.y = max_y;
        } else if position.y > max_y {
            position.y = 1;
        }

        Blizzard { position, ..*self }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Blizzards(Vec<Blizzard>);

impl Blizzards {
    fn new(input: &str) -> Self {
        let input = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let blizzards = (1..input[0].len() - 1)
            .cartesian_product(1..input.len() - 1)
            .filter_map(|(x, y)| {
                Some(Blizzard::new(
                    Position::new(x as i32, y as i32),
                    Direction::new(input[y][x])?,
                ))
            })
            .collect_vec();

        Blizzards(blizzards)
    }
}

struct BlizzardMap(Vec<HashSet<Position>>);

impl BlizzardMap {
    fn new(input: &str) -> Self {
        let max_x = input.lines().next().unwrap().len() as i32 - 2;
        let max_y = input.lines().count() as i32 - 2;

        let mut blizzards = Blizzards::new(input);
        let initial = blizzards.clone();
        let mut blizzard_map = vec![];
        loop {
            blizzard_map.push(blizzards.clone().0.iter().map(|b| b.position).collect());
            blizzards = Blizzards(
                blizzards
                    .0
                    .into_iter()
                    .map(|b| b.step(max_x, max_y))
                    .collect_vec(),
            );
            if blizzards == initial {
                break;
            }
        }

        BlizzardMap(blizzard_map)
    }
}

#[derive(Clone, Copy)]
enum Action {
    DoNothing,
    Move(Direction),
}

impl Action {
    fn all_possible() -> [Self; 5] {
        [
            Action::Move(Direction::Up),
            Action::Move(Direction::Down),
            Action::Move(Direction::Left),
            Action::Move(Direction::Right),
            Action::DoNothing,
        ]
    }
}

#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct State {
    position: Position,
    blizzard_map_index: usize,
    blizzard_map_len: usize,
    max_x: i32,
    max_y: i32,
    has_visited_end: bool,
    has_revisited_start: bool,
}

impl State {
    fn new(input: &str) -> Self {
        let position = Position::new(1, 0);
        let max_x = input.lines().next().unwrap().len() as i32 - 2;
        let max_y = input.lines().count() as i32 - 2;

        State {
            position,
            blizzard_map_index: 0,
            blizzard_map_len: lcm(max_x, max_y) as usize,
            max_x,
            max_y,
            has_visited_end: false,
            has_revisited_start: false,
        }
    }

    fn is_valid(&self, blizzard_map: &BlizzardMap) -> bool {
        if self.position.x == 0 {
            return false;
        }
        if self.position.x > self.max_x {
            return false;
        }
        if self.position.y == 0 && self.position.x != 1 {
            return false;
        }
        if self.position.y > self.max_y && self.position.x != self.max_x {
            return false;
        }
        let occupied = &blizzard_map.0[self.blizzard_map_index];
        if occupied.contains(&self.position) {
            return false;
        }

        true
    }

    fn apply_action(&self, action: Action, blizzard_map: &BlizzardMap) -> Option<Self> {
        let position = match action {
            Action::DoNothing => self.position,
            Action::Move(direction) => self.position.step(direction),
        };

        let blizzard_map_index = (self.blizzard_map_index + 1).rem_euclid(self.blizzard_map_len);

        let has_visited_end = self.has_visited_end
            || (self.position.x == self.max_x && self.position.y == self.max_y + 1);

        let has_revisited_start = self.has_revisited_start
            || (self.position.x == 1 && self.position.y == 0 && self.has_visited_end);

        let new_state = State {
            position,
            blizzard_map_index,
            has_visited_end,
            has_revisited_start,
            ..*self
        };

        if !new_state.is_valid(blizzard_map) {
            return None;
        }

        Some(new_state)
    }

    fn expected_future_cost(&self, goal_position: Position) -> u32 {
        self.position.manhattan_distance(goal_position)
    }

    fn is_goal(&self, goal_position: Position) -> bool {
        self.position == goal_position
    }
}

fn get_time(
    initial_state: State,
    blizzard_map: &BlizzardMap,
    goal_position: Position,
) -> (State, u32) {
    let mut frontier: PriorityQueue<(State, u32), Reverse<u32>> =
        PriorityQueue::from(vec![((initial_state, 0), Reverse(0))]);

    let mut visited = HashSet::new();

    while let Some(((current_state, current_time), _)) = frontier.pop() {
        if !visited.insert(current_state.clone()) {
            continue;
        }

        if current_state.is_goal(goal_position) {
            return (current_state, current_time);
        }

        let possible_actions = Action::all_possible();
        let next_states = possible_actions
            .into_iter()
            .flat_map(|a| current_state.apply_action(a, blizzard_map))
            .collect_vec();

        for next_state in next_states {
            let time = current_time + 1;
            let expected_future_time = next_state.expected_future_cost(goal_position);
            frontier.push_increase((next_state, time), Reverse(time + expected_future_time));
        }
    }

    panic!("no valid path found")
}

fn get_time_with_revisit_start(initial_state: State, blizzard_map: &BlizzardMap) -> u32 {
    let mut total = 0;
    let start_position = Position::new(1, 0);
    let end_position = Position::new(initial_state.max_x, initial_state.max_y + 1);

    let (initial_state, time) = get_time(initial_state, blizzard_map, end_position);
    total += time;
    let (initial_state, time) = get_time(initial_state, blizzard_map, start_position);
    total += time;
    let (_, time) = get_time(initial_state, blizzard_map, end_position);
    total += time;

    total
}

fn main() {
    let input = read_to_string("input").unwrap();
    let initial_state = State::new(&input);
    let blizzard_map = BlizzardMap::new(&input);
    let end_position = Position::new(initial_state.max_x, initial_state.max_y + 1);
    let (_, output_1) = get_time(initial_state.clone(), &blizzard_map, end_position);
    let output_2 = get_time_with_revisit_start(initial_state, &blizzard_map);

    println!("part 1: {output_1} part 2: {output_2}");
}
