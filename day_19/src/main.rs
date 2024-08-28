use std::cmp::max;
use std::collections::HashSet;
use std::fs::read_to_string;

use kdam::TqdmIterator;
use regex::Regex;

enum Action {
    DoNothing,
    BuildOreCollecting,
    BuildClayCollecting,
    BuildGeodeCracking,
    BuildObsidianCollecting,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

impl Resources {
    fn new() -> Self {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Robots {
    ore_collecting: u16,
    clay_collecting: u16,
    obsidian_collecting: u16,
    geode_cracking: u16,
}

impl Robots {
    fn new() -> Self {
        Robots {
            ore_collecting: 1,
            clay_collecting: 0,
            obsidian_collecting: 0,
            geode_cracking: 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    remaining_time: u16,
    resources: Resources,
    robots: Robots,
}

impl State {
    fn new(remaining_time: u16) -> Self {
        State {
            remaining_time,
            resources: Resources::new(),
            robots: Robots::new(),
        }
    }

    fn get_max_geodes(
        &self,
        max_so_far: &mut u16,
        current_geodes: u16,
        blueprint: &Blueprint,
        visited: &mut HashSet<State>,
    ) -> u16 {
        *max_so_far = max(*max_so_far, current_geodes);
        if !visited.insert(self.clone()) {
            return *max_so_far;
        }
        if self.remaining_time == 0 {
            return *max_so_far;
        };
        if self.get_potential() + current_geodes <= *max_so_far {
            return *max_so_far;
        };

        for action in self.get_possible_actions(blueprint) {
            let (new_state, geodes) = self.apply_action(action, blueprint);
            let new_geodes =
                new_state.get_max_geodes(max_so_far, current_geodes + geodes, blueprint, visited);

            *max_so_far = max(*max_so_far, new_geodes);
        }

        *max_so_far
    }

    fn get_possible_actions(&self, blueprint: &Blueprint) -> Vec<Action> {
        let mut output = vec![];
        let max_ore_cost = [
            blueprint.obsidian_robot_cost.0,
            blueprint.ore_robot_cost,
            blueprint.clay_robot_cost,
            blueprint.geode_robot_cost.0,
        ]
        .into_iter()
        .max()
        .unwrap();
        if self.resources.ore >= blueprint.geode_robot_cost.0
            && self.resources.obsidian >= blueprint.geode_robot_cost.1
        {
            output.push(Action::BuildGeodeCracking);
        };
        if self.resources.ore >= blueprint.obsidian_robot_cost.0
            && self.resources.clay >= blueprint.obsidian_robot_cost.1
            && self.robots.obsidian_collecting < blueprint.geode_robot_cost.1
        {
            output.push(Action::BuildObsidianCollecting);
        };
        if self.resources.ore >= blueprint.clay_robot_cost
            && self.robots.clay_collecting < blueprint.obsidian_robot_cost.1
        {
            output.push(Action::BuildClayCollecting);
        };
        if self.resources.ore >= blueprint.ore_robot_cost
            && self.robots.ore_collecting < max_ore_cost
        {
            output.push(Action::BuildOreCollecting);
        };

        if output.len() < 4 {
            output.push(Action::DoNothing);
        }

        output
    }

    fn apply_action(&self, action: Action, blueprint: &Blueprint) -> (Self, u16) {
        let mut new_state = self.clone();
        let mut geodes = 0;
        new_state.resources.ore += new_state.robots.ore_collecting;
        new_state.resources.clay += new_state.robots.clay_collecting;
        new_state.resources.obsidian += new_state.robots.obsidian_collecting;
        new_state.remaining_time -= 1;

        match action {
            Action::BuildGeodeCracking => {
                new_state.resources.ore -= blueprint.geode_robot_cost.0;
                new_state.resources.obsidian -= blueprint.geode_robot_cost.1;
                new_state.robots.geode_cracking += 1;
                geodes = new_state.remaining_time;
            }

            Action::BuildClayCollecting => {
                new_state.resources.ore -= blueprint.clay_robot_cost;
                new_state.robots.clay_collecting += 1;
            }

            Action::BuildObsidianCollecting => {
                new_state.resources.ore -= blueprint.obsidian_robot_cost.0;
                new_state.resources.clay -= blueprint.obsidian_robot_cost.1;
                new_state.robots.obsidian_collecting += 1;
            }

            Action::BuildOreCollecting => {
                new_state.resources.ore -= blueprint.ore_robot_cost;
                new_state.robots.ore_collecting += 1;
            }

            Action::DoNothing => {}
        };

        (new_state, geodes)
    }

    fn get_potential(&self) -> u16 {
        self.remaining_time * (self.remaining_time - 1) / 2
    }
}

struct Blueprint {
    id: u16,
    ore_robot_cost: u16,
    clay_robot_cost: u16,
    obsidian_robot_cost: (u16, u16),
    geode_robot_cost: (u16, u16),
}

impl Blueprint {
    fn get_quality_level(&self, time: u16) -> u16 {
        let max_number_of_geodes = self.get_max_geodes(time);
        max_number_of_geodes * self.id
    }

    fn get_max_geodes(&self, time: u16) -> u16 {
        let start_state = State::new(time);
        start_state.get_max_geodes(&mut 0, 0, self, &mut HashSet::new())
    }

    fn new(input: &str) -> Self {
        let pat = Regex::new(r"\d+").unwrap();
        let nums: Vec<u16> = pat
            .find_iter(input)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let id = nums[0];
        let ore_robot_cost = nums[1];
        let clay_robot_cost = nums[2];
        let obsidian_robot_cost = (nums[3], nums[4]);
        let geode_robot_cost = (nums[5], nums[6]);

        Blueprint {
            id,
            obsidian_robot_cost,
            geode_robot_cost,
            ore_robot_cost,
            clay_robot_cost,
        }
    }
}

struct Blueprints(Vec<Blueprint>);

impl Blueprints {
    fn new(input: &str) -> Self {
        let blueprints = Blueprints(input.lines().map(Blueprint::new).collect());
        blueprints
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let blueprints: Blueprints = Blueprints::new(&input);
    let output_1: u16 = blueprints
        .0
        .iter()
        .tqdm()
        .map(|b| b.get_quality_level(24))
        .sum();
    println!("{output_1}");
    let output_2: u16 = blueprints
        .0
        .iter()
        .take(3)
        .tqdm()
        .map(|b| b.get_max_geodes(32))
        .product();
    println!("part 1: {output_1}, part 2: {output_2}")
}
