use anyhow::{bail, Result};
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
        // Remove the last number and its sums
        let oldest = self.buffer.pop_front().unwrap();
        print!("removing: ");
        for &other in self.buffer.iter() {
            let sum = oldest + other;
            print!("{} ", sum);
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
        println!();

        // Check the newcomer
        let is_a_sum = self.sums.contains(&new);
        // Add it to the sums
        print!("adding: ");
        for &other in self.buffer.iter() {
            let sum = new + other;
            print!("{} ", sum);
            self.sums.insert(sum);
        }
        self.buffer.push_back(new);

        println!(
            "\npopped off {}, adding {}\nbuffer: {:?}\nsums: {:?}\n",
            oldest, new, &self.buffer, &self.sums
        );

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

    // not 50, 17
}
