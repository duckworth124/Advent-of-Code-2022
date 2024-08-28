use std::fs::read_to_string;
use crate::parts::{part_1, part_2};

mod parts;

fn main() {
    let input = read_to_string("input").unwrap();
    println!("part 1: {} part 2: {}", part_1(&input), part_2(&input));
}
