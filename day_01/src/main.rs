mod parts;

use crate::parts::part_2;
use parts::part_1;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    println!("part 1: {} part 2: {}", part_1(&input), part_2(&input));
}
