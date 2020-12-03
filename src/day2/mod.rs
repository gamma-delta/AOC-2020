use once_cell::sync::Lazy;
use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("input.txt");

struct Line {
    policy: Policy,
    input: String,
}

impl Line {
    /// Check if this line is valid wrt part 1
    fn check_part1(&self) -> bool {
        let count = self
            .input
            .chars()
            .filter(|&c| c == self.policy.letter)
            .count();
        self.policy.num1 <= count && count <= self.policy.num2
    }

    /// Check if this line is valid wrt part 2
    fn check_part2(&self) -> bool {
        let chars = self.input.chars().collect::<Vec<_>>();
        let num1_matches = chars[self.policy.num1 - 1] == self.policy.letter;
        let num2_matches = chars[self.policy.num2 - 1] == self.policy.letter;
        num1_matches != num2_matches
    }
}

impl FromStr for Line {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\d+)-(\d+) (\w): (\w*)"#).unwrap());
        let caps = REGEX
            .captures(s)
            .ok_or_else(|| String::from("Regex found no captures"))?;
        let min_count = caps[1].parse().map_err(|e: ParseIntError| e.to_string())?;
        let max_count = caps[2].parse().map_err(|e: ParseIntError| e.to_string())?;
        let letter = caps[3].chars().next().unwrap();
        Ok(Line {
            policy: Policy::new(min_count, max_count, letter),
            input: String::from(&caps[4]),
        })
    }
}

struct Policy {
    /// The first number in a line
    num1: usize,
    /// The second number in a line
    num2: usize,
    letter: char,
}

impl Policy {
    fn new(min_count: usize, max_count: usize, letter: char) -> Self {
        Self {
            num1: min_count,
            num2: max_count,
            letter,
        }
    }
}

#[test]
fn part1() {
    let lines: Vec<Line> = INPUT
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let count = lines.iter().filter(|line| line.check_part1()).count();
    println!("count: {}", count);
}

#[test]
fn part2() {
    let lines: Vec<Line> = INPUT
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let count = lines.iter().filter(|line| line.check_part2()).count();
    println!("count: {}", count);
}
