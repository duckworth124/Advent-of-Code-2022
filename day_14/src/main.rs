use itertools::{iproduct, izip};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

struct Cave {
    has_floor: bool,
    positions: HashSet<Position>,
    max_y: i32,
}

impl Cave {
    fn is_blocked(&self, position: Position) -> bool {
        if self.positions.contains(&position) {
            return true;
        }
        self.has_floor && position.y >= self.max_y + 2
    }

    fn will_fall_forever(&self, position: Position) -> bool {
        !self.has_floor && position.y >= self.max_y
    }
}
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 500, y: -1 }
    }
}

impl Position {
    fn step(self, cave: &Cave) -> Option<Self> {
        let mut new_position = Position {
            y: self.y + 1,
            ..self
        };

        if !cave.is_blocked(new_position) {
            return Some(new_position);
        };

        new_position.x -= 1;
        if !cave.is_blocked(new_position) {
            return Some(new_position);
        };

        new_position.x += 2;
        if !cave.is_blocked(new_position) {
            return Some(new_position);
        };

        None
    }

    fn path(self, cave: &Cave) -> Path {
        Path {
            current_position: self,
            cave,
        }
    }

    fn get_between_points(&self, other: Self) -> impl Iterator<Item = Position> {
        let (min_x, max_x) = (min(self.x, other.x), max(self.x, other.x));
        let (min_y, max_y) = (min(self.y, other.y), max(self.y, other.y));

        iproduct!(min_x..=max_x, min_y..=max_y).map(Position::new)
    }

    fn new((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
}

#[derive(Clone)]
struct Path<'a> {
    cave: &'a Cave,
    current_position: Position,
}

impl<'a> Iterator for Path<'a> {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        self.current_position = self.current_position.step(self.cave)?;
        Some(self.current_position)
    }
}

impl<'a> From<&'a Cave> for Path<'a> {
    fn from(value: &'a Cave) -> Self {
        Position::default().path(value)
    }
}

impl<'a> Path<'a> {
    fn end_point(&mut self) -> Option<Position> {
        if self.clone().any(|p| self.cave.will_fall_forever(p)) {
            None
        } else {
            self.last()
        }
    }
}

fn get_cave(input: &str, has_floor: bool) -> Cave {
    let positions: HashSet<Position> = input
        .lines()
        .flat_map(|line| {
            let positions = line.split(" -> ").map(|s| {
                let [x, y] = s.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>()[0..2]
                else {
                    panic!()
                };
                Position { x, y }
            });
            izip!(positions.clone().skip(1), positions)
                .flat_map(|(p1, p2)| p1.get_between_points(p2))
        })
        .collect();

    Cave {
        max_y: positions.iter().map(|p| p.y).max().unwrap(),
        positions,
        has_floor,
    }
}

fn count_grains(mut cave: Cave) -> u32 {
    let mut current_grain: Path = (&cave).into();
    let mut grain_count = 0;
    while let Some(end) = current_grain.end_point() {
        cave.positions.insert(end);
        grain_count += 1;
        if end.y == 0 {
            break;
        }
        current_grain = (&cave).into();
    }
    grain_count
}

fn main() {
    let input = read_to_string("input").unwrap();
    let cave_without_floor = get_cave(&input, false);
    let cave_with_floor = get_cave(&input, true);

    let output_1 = count_grains(cave_without_floor);
    let output_2 = count_grains(cave_with_floor);

    println!("part 1: {output_1} part 2: {output_2}");
}
