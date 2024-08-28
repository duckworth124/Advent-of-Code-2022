use itertools::{chain, Itertools};
use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Clone)]
enum Token {
    OpenBracket,
    CloseBracket,
    Integer(u32),
}

struct Tokens(Vec<Token>);

impl<T: Into<String>> From<T> for Tokens {
    fn from(value: T) -> Self {
        let value = value.into();
        Tokens({
            value
                .split(',')
                .flat_map(|s| {
                    let open_bracket_count = s.chars().take_while(|&c| c == '[').count();
                    let close_bracket_count = s.chars().skip_while(|&c| c != ']').count();

                    let value = if s.trim_matches(|c| c == '[' || c == ']').is_empty() {
                        vec![]
                    } else {
                        vec![Token::Integer(
                            s.trim_matches(|c| c == '[' || c == ']').parse().unwrap(),
                        )]
                    };

                    chain!(
                        vec![Token::OpenBracket; open_bracket_count],
                        value,
                        vec![Token::CloseBracket; close_bracket_count]
                    )
                })
                .collect()
        })
    }
}

#[derive(Eq, Clone)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl From<Tokens> for Packet {
    fn from(Tokens(value): Tokens) -> Self {
        let mut packets: Vec<Packet> = vec![];
        for token in value {
            match token {
                Token::Integer(x) => {
                    if let Packet::List(v) = packets.last_mut().unwrap() {
                        v.push(Packet::Integer(x))
                    } else {
                        panic!("Integer packet on stack")
                    }
                }

                Token::OpenBracket => packets.push(Packet::new()),

                Token::CloseBracket => {
                    let finished_packet = packets.pop().unwrap();
                    match packets.last_mut() {
                        None => return finished_packet,

                        Some(Packet::List(v)) => v.push(finished_packet),

                        _ => {
                            panic!("Integer packet on stack")
                        }
                    };
                }
            }
        }

        packets.into_iter().next().unwrap()
    }
}

impl<T: Into<String>> From<T> for Packet {
    fn from(value: T) -> Self {
        Packet::from(Tokens::from(value))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => x.cmp(y),
            (Packet::List(v1), Packet::List(v2)) => v1.cmp(v2),
            (Packet::Integer(x), Packet::List(v)) => vec![Packet::Integer(*x)].cmp(v),
            (p1, p2) => p2.cmp(p1).reverse(),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => x.eq(y),
            (Packet::List(v1), Packet::List(v2)) => v1.eq(v2),
            (Packet::Integer(x), Packet::List(v)) => vec![Packet::Integer(*x)].eq(v),
            (p1, p2) => p2.eq(p1),
        }
    }
}

impl Packet {
    fn new() -> Self {
        Packet::List(vec![])
    }
}

fn main() {
    let input = read_to_string("input").unwrap();

    let pairs = get_packet_pairs(&input);
    let packets = get_packets(&input);

    let output_1 = get_unordered_indices(pairs);
    let output_2 = get_decoder_key(packets);

    println!("part 1: {output_1} part 2: {output_2}");
}

fn get_decoder_key(packets: Vec<Packet>) -> usize {
    let divider_packets = ["[[2]]".into(), "[[6]]".into()];
    chain!(packets, divider_packets.clone())
        .sorted()
        .enumerate()
        .filter(|(_, p)| divider_packets.contains(p))
        .map(|(i, _)| i + 1)
        .product()
}
fn get_unordered_indices(pairs: Vec<(Packet, Packet)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1 <= p2)
        .map(|(i, _)| i + 1)
        .sum()
}
fn get_packet_pairs(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|s| {
            let lines: Vec<&str> = s.lines().collect();
            (lines[0].into(), lines[1].into())
        })
        .collect()
}

fn get_packets(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect()
}
