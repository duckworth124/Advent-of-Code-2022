use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

#[derive(Clone, Debug)]
enum ItemValue {
    Normal(u32),
    Big(HashMap<u32, u32>),
}

impl ItemValue {
    fn new(n: u32, divisors: Vec<u32>) -> Self {
        ItemValue::Big(divisors.into_iter().map(|d| (d, n.rem_euclid(d))).collect())
    }
}

impl<T: Into<u32>> std::ops::Add<T> for ItemValue {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        match self {
            ItemValue::Normal(x) => ItemValue::Normal(x + rhs),

            ItemValue::Big(remainders) => ItemValue::Big(
                remainders
                    .iter()
                    .map(|(&divisor, &remainder)| (divisor, (remainder + rhs).rem_euclid(divisor)))
                    .collect(),
            ),
        }
    }
}

impl<T: Into<u32>> std::ops::Mul<T> for ItemValue {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        match self {
            ItemValue::Normal(x) => ItemValue::Normal(x * rhs),

            ItemValue::Big(remainders) => ItemValue::Big(
                remainders
                    .iter()
                    .map(|(&divisor, &remainder)| (divisor, (remainder * rhs).rem_euclid(divisor)))
                    .collect(),
            ),
        }
    }
}

impl From<ItemValue> for u32 {
    fn from(item_value: ItemValue) -> Self {
        match item_value {
            ItemValue::Normal(x) => x,
            ItemValue::Big(remainders) => {
                remainders
                    .into_iter()
                    .reduce(|(divisor_1, remainder_1), (divisor_2, remainder_2)| {
                        (
                            lcm(divisor_1, divisor_2),
                            (0..divisor_2)
                                .map(|n| divisor_1 * n + remainder_1)
                                .find(|&n| n.rem_euclid(divisor_2) == remainder_2)
                                .unwrap(),
                        )
                    })
                    .unwrap()
                    .1
            }
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

impl Operation {
    fn apply(&self, input: ItemValue) -> ItemValue {
        match self {
            Operation::Add(other) => input + *other,
            Operation::Mul(other) => input * *other,
            Operation::Square => input.clone() * input,
        }
    }
}

#[derive(Clone)]
struct DivisibilityTest {
    divisor: u32,
    if_true: usize,
    if_false: usize,
}

impl DivisibilityTest {
    fn apply(&self, input: &ItemValue) -> usize {
        let result = match input {
            ItemValue::Big(remainders) => remainders
                .get(&self.divisor)
                .map(|&n| n == 0)
                .unwrap_or(false),

            ItemValue::Normal(n) => n.rem_euclid(self.divisor) == 0,
        };

        if result {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone)]
struct Monkey {
    activity: u64,
    items: VecDeque<ItemValue>,
    operation: Operation,
    divisibility_test: DivisibilityTest,
}

fn main() {
    let input = read_to_string("input").unwrap();

    let monkeys_1 = process_input(&input, true);
    let monkeys_2 = process_input(&input, false);

    let output_1 = simulate_simians(monkeys_1, 20, true);
    let output_2 = simulate_simians(monkeys_2, 10000, false);

    println!("part 1: {output_1} part 2: {output_2}");
}

fn process_input(input: &str, divide_worry_levels: bool) -> Vec<Monkey> {
    let divisors = if divide_worry_levels {
        None
    } else {
        Some(
            input
                .lines()
                .map(|s| s.trim())
                .filter_map(|s| s.strip_prefix("Test: divisible by "))
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    };
    input
        .split("\n\n")
        .map(|s| generate_monkey(s, divisors.clone()))
        .collect()
}

fn round(monkeys: &mut [Monkey], divide_worry_level: bool) {
    (0..monkeys.len()).for_each(|i| turn(monkeys, i, divide_worry_level))
}

fn turn(monkeys: &mut [Monkey], monkey_index: usize, divide_worry_level: bool) {
    let monkey = monkeys.get_mut(monkey_index).unwrap();
    let mut items_to_throw: Vec<(ItemValue, usize)> = vec![];
    while let Some(mut item_value) = monkey.items.pop_front() {
        item_value = monkey.operation.apply(item_value);
        if divide_worry_level {
            if let ItemValue::Normal(x) = item_value {
                item_value = ItemValue::Normal(x / 3)
            }
        }
        let next_monkey_index = monkey.divisibility_test.apply(&item_value);
        items_to_throw.push((item_value, next_monkey_index));
        monkey.activity += 1;
    }

    for (item_value, next_monkey_index) in items_to_throw {
        monkeys[next_monkey_index].items.push_back(item_value)
    }
}

fn generate_monkey(input: &str, divisors: Option<Vec<u32>>) -> Monkey {
    let lines = input.lines().collect_vec();
    let items: VecDeque<ItemValue> = get_starting_items(lines[1], &divisors);
    let operation = get_operation(lines[2]);
    let divisibility_test = get_divisibility_test(&lines[3..]);

    Monkey {
        activity: 0,
        items,
        operation,
        divisibility_test,
    }
}

fn get_starting_items(input: &str, divisors: &Option<Vec<u32>>) -> VecDeque<ItemValue> {
    input
        .trim()
        .strip_prefix("Starting items: ")
        .unwrap()
        .split(", ")
        .map(|s| s.parse().unwrap())
        .map(|n| match divisors {
            None => ItemValue::Normal(n),
            Some(divisors) => ItemValue::new(n, divisors.clone()),
        })
        .collect()
}

fn get_operation(input: &str) -> Operation {
    let text_to_process = input.trim().strip_prefix("Operation: new = ").unwrap();

    if text_to_process == "old * old" {
        return Operation::Square;
    };

    if text_to_process.starts_with("old *") {
        return Operation::Mul(
            text_to_process
                .strip_prefix("old * ")
                .unwrap()
                .parse()
                .unwrap(),
        );
    };

    Operation::Add(
        text_to_process
            .strip_prefix("old + ")
            .unwrap()
            .parse()
            .unwrap(),
    )
}

fn get_divisibility_test(input: &[&str]) -> DivisibilityTest {
    let divisor: u32 = input[0]
        .trim()
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();

    let if_true = input[1]
        .trim()
        .strip_prefix("If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    let if_false = input[2]
        .trim()
        .strip_prefix("If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    DivisibilityTest {
        divisor,
        if_true,
        if_false,
    }
}

fn simulate_simians(mut monkeys: Vec<Monkey>, round_count: u32, divide_worry_level: bool) -> u64 {
    for _ in 0..round_count {
        round(&mut monkeys, divide_worry_level);
    }
    let monkey_business: u64 = monkeys.iter().map(|m| m.activity).k_largest(2).product();

    monkey_business
}
