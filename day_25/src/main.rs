use std::{fmt::Display, fs::read_to_string, iter::Sum, ops::Add};

struct SnafuNumber {
    digits: Vec<i32>,
}

impl SnafuNumber {
    fn new() -> Self {
        let digits = vec![];
        Self { digits }
    }
}

impl From<&str> for SnafuNumber {
    fn from(value: &str) -> Self {
        let digits = value
            .chars()
            .rev()
            .map(|c| match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid character"),
            })
            .collect();

        Self { digits }
    }
}

impl Add for SnafuNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = self.digits.len().max(rhs.digits.len()) + 1;
        let mut digits = vec![];
        let mut carry = 0;
        for i in 0..len {
            let mut sum =
                self.digits.get(i).unwrap_or(&0) + rhs.digits.get(i).unwrap_or(&0) + carry;

            if sum > 2 {
                sum -= 5;
                carry = 1;
            } else if sum < -2 {
                sum += 5;
                carry = -1;
            } else {
                carry = 0
            }

            digits.push(sum)
        }

        while digits.last() == Some(&0) {
            digits.pop();
        }

        Self { digits }
    }
}

impl Sum for SnafuNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(SnafuNumber::new(), |acc, n| acc + n)
    }
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .rev()
                .map(|i| match i {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("invalid digit"),
                })
                .collect::<String>()
        )
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output: SnafuNumber = input.lines().map(SnafuNumber::from).sum();
    println!("{output}")
}
