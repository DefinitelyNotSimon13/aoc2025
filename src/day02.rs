#![deny(clippy::pedantic)]
use std::num::ParseIntError;

use aoc2025::{AoCResult, aoc_day};

const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659, 824824821-824824827,2121212118-2121212124\n";

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl TryFrom<&str> for Range {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let nums: Vec<&str> = value.trim().split("-").collect();

        let start: u64 = nums.get(0).expect("get range start").parse()?;
        let end: u64 = nums.get(1).expect("get parse end").parse()?;

        Ok(Self { start, end })
    }
}

impl Range {
    pub fn get_invalid_ids_part1(&self) -> Vec<u64> {
        let mut invalid_ids = Vec::new();
        for i in self.start..=self.end {
            let id_str = i.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }

            if is_invalid(&id_str) {
                invalid_ids.push(i);
            }
        }

        invalid_ids
    }

    pub fn get_invalid_ids_part2(&self) -> Vec<u64> {
        let mut invalid_ids = Vec::new();
        'outer: for i in self.start..=self.end {
            let id_str = i.to_string();

            let sequence = shortest_repeating_prefix(&id_str);

            if id_str.len() % sequence.len() != 0 || id_str.len() == sequence.len() {
                continue;
            }

            let mut it = id_str.chars().peekable();
            while it.peek().is_some() {
                let chunk: String = it.by_ref().take(sequence.len()).collect();
                if chunk.len() < sequence.len() {
                    continue 'outer;
                }

                if chunk != sequence {
                    continue 'outer;
                }
            }

            invalid_ids.push(i);
        }

        invalid_ids
    }
}

fn shortest_repeating_prefix(s: &str) -> &str {
    let bytes = s.as_bytes();
    let n = bytes.len();

    for len in 1..=n {
        if n % len != 0 {
            continue;
        }

        let prefix = &bytes[..len];

        if bytes.chunks(len).all(|chunk| chunk == prefix) {
            return &s[..len];
        }
    }
    s
}
fn is_invalid(s: &str) -> bool {
    assert!(s.len() % 2 == 0);
    let middle = s.len() / 2;

    let (first, second) = s.split_at(middle);
    first == second
}

fn get_ranges(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(",")
        .map(Range::try_from)
        .map(|r| r.expect("get range from str"))
        .collect()
}

fn part1(input: &str) -> AoCResult<u64> {
    Ok(get_ranges(input)
        .iter()
        .map(Range::get_invalid_ids_part1)
        .flatten()
        .sum())
}

fn part2(input: &str) -> AoCResult<u64> {
    Ok(get_ranges(input)
        .iter()
        .map(Range::get_invalid_ids_part2)
        .flatten()
        .sum())
}

aoc_day!(
    2,
    "Gift Shop",
    example = {
        label: "Example",
        input: EXAMPLE_INPUT,
        expected1: Some(1227775554u64),
        expected2: Some(4174379265),
    }
);
