use itertools::{izip, Itertools};
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new_priorities() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }

    fn get_orthogonal(&self) -> [Self; 2] {
        if self.is_horizontal() {
            [Direction::Up, Direction::Down]
        } else {
            [Direction::Left, Direction::Right]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
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

        let (x, y) = (self.x + dx, self.y + dy);
        Position { x, y }
    }

    fn get_forward_and_diagonal(&self, direction: Direction) -> [Position; 3] {
        let forward = self.step(direction);
        let orthogonal_directions = direction.get_orthogonal();
        let diagonals = orthogonal_directions.map(|d| forward.step(d));
        [forward, diagonals[0], diagonals[1]]
    }

    fn is_clear(&self, direction: Direction, occupied: &HashSet<Position>) -> bool {
        self.get_forward_and_diagonal(direction)
            .into_iter()
            .all(|p| !occupied.contains(&p))
    }

    fn is_isolated(&self, occupied: &HashSet<Position>) -> bool {
        Direction::new_priorities()
            .into_iter()
            .all(|d| self.is_clear(d, occupied))
    }
}

struct Agent {
    position: Position,
}

impl Agent {
    fn new(x: i32, y: i32) -> Self {
        let position = Position::new(x, y);
        Agent { position }
    }

    fn get_proposed_movement(
        &self,
        occupied: &HashSet<Position>,
        direction_priorities: &VecDeque<Direction>,
    ) -> Option<Direction> {
        if self.position.is_isolated(occupied) {
            return None;
        }

        direction_priorities
            .iter()
            .copied()
            .find(|d| self.position.is_clear(*d, occupied))
    }
}

struct Agents(Vec<Agent>);

impl Agents {
    fn new(input: &str) -> Self {
        let agents = input
            .lines()
            .map(|line| line.char_indices())
            .enumerate()
            .flat_map(|(i, c)| [i].into_iter().cycle().zip(c).collect_vec())
            .filter(|(_, (_, c))| *c == '#')
            .map(|(y, (x, _))| Agent::new(x as i32, y as i32))
            .collect();

        Agents(agents)
    }

    fn iter(&self) -> impl Iterator<Item = &Agent> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Agent> {
        self.0.iter_mut()
    }
}

struct State {
    agents: Agents,
    direction_priorites: VecDeque<Direction>,
    occupied: HashSet<Position>,
}

impl State {
    fn new(input: &str) -> Self {
        let agents = Agents::new(input);
        let direction_priorites = VecDeque::from(Direction::new_priorities());
        let occupied = agents.iter().map(|a| a.position).collect();
        State {
            agents,
            direction_priorites,
            occupied,
        }
    }

    fn get_proposed_movements(&self) -> Vec<Option<Direction>> {
        self.agents
            .iter()
            .map(|a| a.get_proposed_movement(&self.occupied, &self.direction_priorites))
            .collect_vec()
    }

    fn apply_proposed_movements(&mut self, proposed_movements: Vec<Option<Direction>>) -> bool {
        let proposed_destinations = self
            .agents
            .iter()
            .zip(proposed_movements)
            .map(|(a, d)| d.map(|direction| a.position.step(direction)))
            .collect_vec();

        let counts = proposed_destinations
            .iter()
            .map(|p| proposed_destinations.iter().filter(|x| *x == p).count())
            .collect_vec();

        let is_not_conflicting = proposed_destinations
            .iter()
            .zip(counts)
            .map(|(_, c)| c == 1)
            .collect_vec();

        let mut did_move = false;
        for (agent, destination, _) in izip!(
            self.agents.iter_mut(),
            proposed_destinations,
            is_not_conflicting
        )
        .filter(|(_, _, b)| *b)
        {
            if let Some(p) = destination {
                self.occupied.remove(&agent.position);
                agent.position = p;
                self.occupied.insert(p);
                did_move = true
            }
        }

        did_move
    }

    fn shift_direction_priorites(&mut self) {
        let front = self.direction_priorites.pop_front().unwrap();
        self.direction_priorites.push_back(front);
    }

    fn run_round(&mut self) -> bool {
        let proposed_movements = self.get_proposed_movements();
        let did_move = self.apply_proposed_movements(proposed_movements);
        self.shift_direction_priorites();
        did_move
    }

    fn count_empty_in_rectangle(&self) -> usize {
        let min_x = self.agents.iter().map(|a| a.position.x).min().unwrap();
        let max_x = self.agents.iter().map(|a| a.position.x).max().unwrap();
        let min_y = self.agents.iter().map(|a| a.position.y).min().unwrap();
        let max_y = self.agents.iter().map(|a| a.position.y).max().unwrap();

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .map(|(x, y)| Position::new(x, y))
            .filter(|&p| !self.occupied.contains(&p))
            .count()
    }
}
fn main() {
    let input = read_to_string("input").unwrap();
    let mut state = State::new(&input);
    for _ in 0..10 {
        state.run_round();
    }

    let output_1 = state.count_empty_in_rectangle();

    let mut state = State::new(&input);
    let mut round_number = 1;
    while state.run_round() {
        round_number += 1;
    }

    let output_2 = round_number;
    println!("part 1: {output_1} part 2: {output_2}")
}
