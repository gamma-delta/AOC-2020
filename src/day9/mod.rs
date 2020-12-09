use anyhow::{bail, Result};
use itertools::Itertools;
use multiset::HashMultiSet;

use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

struct Decoder {
    /// Buffer of received numbers.
    /// New numbers are pushed into the back and old ones are popped off the front.
    buffer: VecDeque<i64>,

    /// Set of all numbers the buffer can sum to.
    sums: HashMultiSet<i64>,

    /// Size of the buffer (or, the length of the preamble)
    bufsize: usize,
}

impl Decoder {
    /// Create a new Decoder from a buffer
    pub fn new(preamble: &[i64]) -> Self {
        let mut sums = HashMultiSet::new();
        for i in 0..preamble.len() {
            for j in (i + 1)..preamble.len() {
                sums.insert(preamble[i] + preamble[j]);
            }
        }

        Self {
            buffer: preamble.iter().cloned().collect(),
            sums,
            bufsize: preamble.len(),
        }
    }

    /// Receive a new number. Returns whether it was the sum of two previous numbers. (or an error)
    pub fn receive(&mut self, new: i64) -> Result<bool> {
        let is_a_sum = self.sums.contains(&new);

        // Remove the last number and its sums
        let oldest = self.buffer.pop_front().unwrap();
        for &other in self.buffer.iter() {
            let sum = oldest + other;
            let existed = self.sums.remove(&sum);
            if !existed {
                bail!(
                    "when popping off {}, its sum with {} ({}) was not in the set",
                    oldest,
                    other,
                    oldest + other
                );
            }
        }
        // Add it to the sums
        for &other in self.buffer.iter() {
            let sum = new + other;
            self.sums.insert(sum);
        }
        self.buffer.push_back(new);

        Ok(is_a_sum)
    }
}

#[test]
fn part1() -> Result<()> {
    let transmission = INPUT
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<Result<Vec<i64>>>()?;
    let (preamble, tail) = transmission.split_at(25);

    let mut decoder = Decoder::new(preamble);
    for &num in tail {
        let sum_ok = decoder.receive(num)?;
        if !sum_ok {
            println!("{} was not present in the sums!", num);
            break;
        }
    }

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let transmission = INPUT
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<Result<Vec<i64>>>()?;
    let (preamble, tail) = transmission.split_at(25);

    let mut decoder = Decoder::new(preamble);
    let invalid = *tail
        .iter()
        .map(|&num| decoder.receive(num).map(|success| (num, success)))
        .map_results(|(num, success)| if !success { Some(num) } else { None })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .find_map(|it| it.as_ref())
        .unwrap();
    println!("our illegal number is {}", invalid);

    for i in 0..transmission.len() {
        for j in (i + 1)..transmission.len() {
            let range = &transmission[i..=j];
            let sum: i64 = range.iter().sum();
            if sum == invalid {
                // poggers
                let (low, high) = range.iter().minmax().into_option().unwrap();
                println!("weakness: {} from [{}, {}]", low + high, i, j);
            }
        }
    }

    Ok(())
}

#[test]
fn part1_test() -> Result<()> {
    let input = &r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let transmission = input
        .lines()
        .map(|line| Ok(line.parse()?))
        .collect::<Result<Vec<i64>>>()?;
    let (preamble, tail) = transmission.split_at(5);

    let mut decoder = Decoder::new(preamble);
    for &num in tail {
        let sum_ok = decoder.receive(num)?;
        if !sum_ok {
            println!("{} was not present in the sums!", num);
            break;
        }
    }

    Ok(())
}
