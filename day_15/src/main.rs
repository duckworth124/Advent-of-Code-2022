use itertools::{chain, Itertools};
use regex::Regex;
use std::fs::read_to_string;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn manhattan_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct Sensor {
    sensor_position: Position,
    beacon_position: Position,
    range: u64,
}

impl Sensor {
    fn distance_to_outside_of_range(&self, position: &Position) -> u64 {
        let distance = position.manhattan_distance(&self.sensor_position);
        (self.range + 1).saturating_sub(distance)
    }

    fn range_border(&self) -> Vec<Position> {
        chain!(
            (0..=self.range + 1).map(|i| Position {
                x: self.sensor_position.x + i as i64,
                y: self.sensor_position.y + (self.range + 1 - i) as i64
            }),
            (0..=self.range + 1).map(|i| Position {
                x: self.sensor_position.x + i as i64,
                y: self.sensor_position.y - (self.range + 1 - i) as i64
            }),
            (0..=self.range + 1).map(|i| Position {
                x: self.sensor_position.x - i as i64,
                y: self.sensor_position.y + (self.range + 1 - i) as i64
            }),
            (0..=self.range + 1).map(|i| Position {
                x: self.sensor_position.x - i as i64,
                y: self.sensor_position.y - (self.range + 1 - i) as i64
            })
        )
        .filter(|p| 0 <= p.x && p.x <= MAX_COORD && 0 <= p.y && p.y <= MAX_COORD)
        .collect()
    }
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let pattern = Regex::new(r"-?\d+").unwrap();
        let nums: Vec<i64> = pattern
            .find_iter(value)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let (sensor_x, sensor_y) = (nums[0], nums[1]);
        let (beacon_x, beacon_y) = (nums[2], nums[3]);

        let sensor_position = Position {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon_position = Position {
            x: beacon_x,
            y: beacon_y,
        };
        let range = sensor_position.manhattan_distance(&beacon_position);

        Sensor {
            sensor_position,
            beacon_position,
            range,
        }
    }
}

struct Sensors(Vec<Sensor>);

impl From<&str> for Sensors {
    fn from(value: &str) -> Self {
        Sensors(value.lines().map(Sensor::from).collect())
    }
}

impl Sensors {
    fn distance_to_outside_of_ranges(&self, position: &Position) -> u64 {
        self.0
            .iter()
            .map(|s| s.distance_to_outside_of_range(position))
            .max()
            .unwrap()
    }

    fn x_range(&self) -> (i64, i64) {
        (
            self.0
                .iter()
                .map(|s| s.sensor_position.x - s.range as i64)
                .min()
                .unwrap(),
            self.0
                .iter()
                .map(|s| s.sensor_position.x + s.range as i64)
                .max()
                .unwrap(),
        )
    }

    fn range_borders(&self) -> Vec<Position> {
        self.0.iter().flat_map(|s| s.range_border()).collect()
    }
}

const Y: i64 = 2000000;
//const Y: i64 = 10;

fn count_covered_in_row(sensors: &Sensors) -> usize {
    let (min_x, max_x) = sensors.x_range();
    let mut current_position = Position { x: min_x, y: Y };
    let mut output: usize = 0;
    while current_position.x <= max_x {
        let distance = sensors.distance_to_outside_of_ranges(&current_position);
        if distance > 0 {
            output += distance as usize;
            current_position.x += distance as i64;
        } else {
            current_position.x += 1
        }
    }
    output -= sensors
        .0
        .iter()
        .filter(|s| s.beacon_position.y == Y)
        .map(|s| s.beacon_position.x)
        .sorted()
        .dedup()
        .count();

    output
}

const MAX_COORD: i64 = 4_000_000;
//const MAX_COORD: i64 = 20;
fn find_tuning_frequency(sensors: &Sensors) -> i64 {
    let points_to_check = sensors.range_borders();
    let beacon_position = points_to_check
        .iter()
        .find(|p| sensors.distance_to_outside_of_ranges(p) == 0)
        .unwrap();

    beacon_position.x * 4_000_000 + beacon_position.y
}

fn main() {
    let input = read_to_string("input").unwrap();
    let sensors: Sensors = input.as_str().into();

    let output_1 = count_covered_in_row(&sensors);
    let output_2 = find_tuning_frequency(&sensors);

    println!("part 1: {output_1} part 2: {output_2}");
}
