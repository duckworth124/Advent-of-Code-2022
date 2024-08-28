use std::{collections::HashSet, fs::read_to_string};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn get_neighbours(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x + 1,
                ..*self
            },
            Position {
                x: self.x - 1,
                ..*self
            },
            Position {
                y: self.y + 1,
                ..*self
            },
            Position {
                y: self.y - 1,
                ..*self
            },
            Position {
                z: self.z + 1,
                ..*self
            },
            Position {
                z: self.z - 1,
                ..*self
            },
        ]
    }

    fn new(input: &str) -> Self {
        let coords = input.split(',').collect::<Vec<_>>();
        let x = coords[0].parse().unwrap();
        let y = coords[1].parse().unwrap();
        let z = coords[2].parse().unwrap();
        Position { x, y, z }
    }
}
struct Droplet {
    positions: HashSet<Position>,
}

impl Droplet {
    fn new(input: &str) -> Self {
        let positions = input.lines().map(Position::new).collect();

        Droplet { positions }
    }

    fn get_external_points(&self) -> HashSet<Position> {
        let min_x = self.positions.iter().map(|p| p.x).min().unwrap() - 1;
        let max_x = self.positions.iter().map(|p| p.x).max().unwrap() + 1;
        let min_y = self.positions.iter().map(|p| p.y).min().unwrap() - 1;
        let max_y = self.positions.iter().map(|p| p.y).max().unwrap() + 1;
        let min_z = self.positions.iter().map(|p| p.z).min().unwrap() - 1;
        let max_z = self.positions.iter().map(|p| p.z).max().unwrap() + 1;

        let mut visited = HashSet::new();
        let mut frontier = vec![Position {
            x: min_x,
            y: min_y,
            z: min_z,
        }];

        while let Some(pos) = frontier.pop() {
            if self.positions.contains(&pos) {
                continue;
            };
            if !visited.insert(pos) {
                continue;
            };

            if pos.x < min_x
                || pos.x > max_x
                || pos.y < min_y
                || pos.y > max_y
                || pos.z < min_z
                || pos.z > max_z
            {
                continue;
            }

            frontier.extend(pos.get_neighbours())
        }

        visited
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let droplet: Droplet = Droplet::new(&input);
    let output_1 = (droplet.positions.iter())
        .flat_map(|p| {
            p.get_neighbours()
                .into_iter()
                .filter(|p_2| !droplet.positions.contains(p_2))
                .collect::<Vec<_>>()
        })
        .count();

    let external_points = droplet.get_external_points();

    let output_2 = droplet
        .positions
        .iter()
        .flat_map(|p| {
            p.get_neighbours()
                .into_iter()
                .filter(|p_2| external_points.contains(p_2))
        })
        .count();

    println!();
    println!("part 1: {output_1} part 2: {output_2}");
}
