#[derive(Eq, PartialEq, Copy, Clone)]
enum Hand {
    Rock=1,
    Paper=2,
    Scissors=3
}

impl Hand {
    fn get_result(&self, other: Hand) -> GameResult {
        if self == &other {
            return GameResult::Draw
        };

        match self {
            Hand::Rock => if other == Hand::Scissors {
                return GameResult::Win
            },

            Hand::Paper => if other == Hand::Rock {
                return GameResult::Win
            },

            Hand::Scissors => if other == Hand::Paper {
                return GameResult::Win
            }
        };

        GameResult::Loss
    }

    fn get_score(&self, other: Hand) -> u32 {
        *self as u32 + self.get_result(other) as u32
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum GameResult {
    Win=6,
    Loss=0,
    Draw=3
}

fn process_input_1(input: &str) -> Vec<(Hand, Hand)> {
    input.lines()
        .map(|line| {
            let my_hand = match line.chars().nth(2).unwrap() {
                'X' => Hand::Rock,
                'Y' => Hand::Paper,
                'Z' => Hand::Scissors,
                _ => panic!("character not recognised")
            };

            let other_hand = match line.chars().next().unwrap() {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissors,
                _ => panic!("character not recognised")
            };

            (my_hand, other_hand)
        })
        .collect()
}

pub fn part_1(input: &str) -> u32 {
    process_input_1(input)
        .iter()
        .map(|&(my_hand, other_hand)| my_hand.get_score(other_hand))
        .sum()
}

fn process_input_2(input: &str) -> Vec<(GameResult, Hand)> {
    input.lines()
        .map(|line| {
            let game_result = match line.chars().nth(2).unwrap() {
                'X' => GameResult::Loss,
                'Y' => GameResult::Draw,
                'Z' => GameResult::Win,
                _ => panic!("character not recognised")
            };

            let other_hand = match line.chars().next().unwrap() {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissors,
                _ => panic!("character not recognised")
            };

            (game_result, other_hand)
        })
        .collect()
}

pub fn part_2(input: &str) -> u32 {
    process_input_2(input)
        .iter()
        .map(|&(|game_result, other_hand)| {
            let to_add = match game_result {
                GameResult::Loss => -1,
                GameResult::Draw => 0,
                GameResult::Win => 1
            };
            let hand_score = ((other_hand as i32 - 1) + to_add).rem_euclid(3) as u32 + 1;
            hand_score + game_result as u32
        })
        .sum()
}