use itertools::{izip, Itertools};
use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone)]
struct Valve {
    flow_rate: u32,
    reachable_valves: HashMap<String, u32>,
}

#[derive(Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Agent {
    position: String,
    sleeping_for: u32,
}

impl Agent {
    fn new(position: &str) -> Self {
        Agent {
            position: position.to_string(),
            sleeping_for: 0,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State {
    unopened_valves: Vec<String>,

    agents: Vec<Agent>,
    remaining_time: u32,
}

impl State {
    fn new(valves: &HashMap<String, Valve>, number_of_agents: usize, remaining_time: u32) -> Self {
        State {
            unopened_valves: valves
                .iter()
                .filter(|(_, v)| v.flow_rate != 0)
                .map(|(s, _)| s)
                .cloned()
                .collect(),
            agents: vec![Agent::new("AA"); number_of_agents],
            remaining_time,
        }
    }

    fn possible_actions(&self) -> Vec<Vec<String>> {
        self.agents
            .iter()
            .map(|a| {
                if a.sleeping_for == 0 {
                    self.unopened_valves.clone()
                } else {
                    vec![a.position.clone()]
                }
            })
            .multi_cartesian_product()
            .collect()
    }

    fn apply_actions(&self, actions: &Vec<String>, valves: &HashMap<String, Valve>) -> (Self, u32) {
        let mut new_state = self.clone();
        let mut pressure_released = 0;
        for (action, agent) in izip!(actions, new_state.agents.iter_mut()) {
            if agent.sleeping_for > 0 {
                continue;
            }
            if !new_state.unopened_valves.contains(action) {
                continue;
            }
            agent.sleeping_for = valves[&agent.position.clone()].reachable_valves[action] + 1;
            agent.position.clone_from(action);
            let remaining_time = self.remaining_time.saturating_sub(agent.sleeping_for);
            let flow_rate = valves[action].flow_rate;
            pressure_released += remaining_time * flow_rate;

            new_state.unopened_valves.swap_remove(
                new_state
                    .unopened_valves
                    .iter()
                    .position(|x| x == action)
                    .unwrap(),
            );
        }

        let sleep_time = new_state
            .agents
            .iter()
            .map(|a| a.sleeping_for)
            .min()
            .unwrap();

        for agent in new_state.agents.iter_mut() {
            agent.sleeping_for = agent.sleeping_for.saturating_sub(sleep_time)
        }

        new_state.remaining_time = new_state.remaining_time.saturating_sub(sleep_time);
        new_state.agents.sort();
        (new_state, pressure_released)
    }

    fn potential(&self, valves: &HashMap<String, Valve>) -> u32 {
        self.unopened_valves
            .iter()
            .map(|s| valves[s].flow_rate)
            .sum::<u32>()
            * (self.remaining_time.saturating_sub(1))
    }
}

struct Valves {
    valves: HashMap<String, Valve>,
}

impl Valves {
    fn new(input: &str) -> Self {
        let valves: HashMap<String, Valve> = input
            .lines()
            .map(|line| {
                let int_pat = Regex::new(r"\d+").unwrap();
                let valve_pat = Regex::new(r"[A-Z][A-Z]").unwrap();
                let flow_rate: u32 = int_pat.find(line).unwrap().as_str().parse().unwrap();
                let current_valve = valve_pat.find(line).unwrap().as_str().to_string();
                let reachable_valves: HashMap<String, u32> = valve_pat
                    .find_iter(line)
                    .skip(1)
                    .map(|m| (m.as_str().to_string(), 1))
                    .collect();

                (
                    current_valve,
                    Valve {
                        flow_rate,
                        reachable_valves,
                    },
                )
            })
            .collect();

        Valves { valves }.get_complete_graph()
    }

    fn get_complete_graph(self) -> Self {
        let mut important_valves: HashMap<String, Valve> = self
            .valves
            .iter()
            .filter(|(s, v)| s == &"AA" || v.flow_rate > 0)
            .map(|(x, y)| (x.clone(), y.clone()))
            .collect();

        for valve in important_valves.values_mut() {
            valve.reachable_valves.clear()
        }

        for (start, end) in important_valves
            .clone()
            .keys()
            .permutations(2)
            .map(|v| (v[0].clone(), v[1].clone()))
        {
            important_valves
                .get_mut(&start)
                .unwrap()
                .reachable_valves
                .insert(end.clone(), self.get_distance(start, end));
        }

        Valves {
            valves: important_valves,
        }
    }

    fn get_distance(&self, start: String, end: String) -> u32 {
        let mut frontier = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::new();
        while let Some((valve_name, distance)) = frontier.pop_front() {
            if !visited.insert(valve_name.clone()) {
                continue;
            }
            if valve_name == end {
                return distance;
            }

            for next in self.valves[&valve_name].reachable_valves.keys() {
                frontier.push_back((next.clone(), distance + 1))
            }
        }
        panic!("no path found")
    }

    fn get_max_pressure(
        &self,
        state: State,
        max_so_far: &mut u32,
        value_of_getting_here: u32,
        visited: &mut HashSet<State>,
    ) -> u32 {
        if !visited.insert(state.clone()) {
            return *max_so_far;
        }
        *max_so_far = max(*max_so_far, value_of_getting_here);
        if state.remaining_time == 0 {
            return *max_so_far;
        };
        if state.potential(&self.valves) + value_of_getting_here <= *max_so_far {
            return *max_so_far;
        };

        for action in state.possible_actions() {
            let (next, value) = state.apply_actions(&action, &self.valves);
            let next_value =
                self.get_max_pressure(next, max_so_far, value_of_getting_here + value, visited);
            *max_so_far = max(*max_so_far, next_value);
        }

        *max_so_far
    }
}

fn main() {
    let timer = Instant::now();
    let input = read_to_string("input").unwrap();
    let cave = Valves::new(&input);

    let output_1 = cave.get_max_pressure(
        State::new(&cave.valves, 1, 30),
        &mut 0,
        0,
        &mut HashSet::new(),
    );
    let output_2 = cave.get_max_pressure(
        State::new(&cave.valves, 2, 26),
        &mut 0,
        0,
        &mut HashSet::new(),
    );
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}", timer.elapsed().as_secs_f32());
}
