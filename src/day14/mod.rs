use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use regex::Regex;

use std::{collections::BTreeMap, convert::identity};

const INPUT: &str = include_str!("input.txt");

/// Program for part 1
struct ProgramP1 {
    /// Has a 1 in every position a mask has a 1 and 0 otherwise.
    ///
    /// ```text
    ///   XX0XX1X0
    /// becomes
    /// + 00000100
    /// - 11011110
    /// ```
    positive_mask: u64,
    /// Has a 0 in every position a mask has a 0 and 1 otherwise.
    negative_mask: u64,
    /// Contents of memory. Maps indices to values.
    memory: BTreeMap<u64, u64>,
}

static MEM_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"mem\[(\d+)\]").unwrap());

impl ProgramP1 {
    /// Make a new default program
    fn new() -> Self {
        Self {
            positive_mask: 0,
            negative_mask: 0xffffffffffffffff,
            memory: BTreeMap::new(),
        }
    }

    /// Accept one line of input (aka one instruction)
    fn input(&mut self, line: &str) -> Result<()> {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let target = split[0];
        let equals_sign = split[1];
        let value = split[2];

        if equals_sign != "=" {
            bail!("expected equals sign");
        }

        match target {
            "mask" => {
                let mut pmask = 0;
                let mut nmask = 0;
                // we can use chars & len here because everything is ascii
                for (ch, place) in value.chars().zip((0..value.len()).rev()) {
                    match ch {
                        'X' => {}
                        '1' => pmask |= 1 << place,
                        '0' => nmask |= 1 << place,
                        oh_no => bail!("unknown char {} in mask", oh_no),
                    }
                }
                self.positive_mask = pmask;
                self.negative_mask = !nmask;
            }
            memory => {
                let caps = MEM_REGEX
                    .captures(memory)
                    .ok_or_else(|| anyhow!("memory regex failed to match on {}", memory))?;
                let addr = caps
                    .get(1)
                    .ok_or_else(|| anyhow!("memory regex didn't have group 1 on {}", memory))?;
                let addr = addr.as_str().parse()?;
                // without the explicit type, rustc thinks it is a unit
                // :thonk:
                let value: u64 = value.parse()?;
                let value = (value | self.positive_mask) & self.negative_mask;
                self.memory.insert(addr, value);
            }
        }

        Ok(())
    }
}

/// Program for part 2
struct ProgramP2 {
    /// The mask.
    ///
    /// `Some(true)` = 1. `Some(false)` = 0. `None` = X
    mask: [Option<bool>; 36],

    /// Contents of memory. Maps indices to values.
    memory: BTreeMap<u64, u64>,
}

impl ProgramP2 {
    fn new() -> Self {
        Self {
            // huh i'm pleasantly surprised this syntax works
            mask: [None; 36],
            memory: BTreeMap::new(),
        }
    }

    /// Accept one line of input (aka one instruction)
    fn input(&mut self, line: &str) -> Result<()> {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let target = split[0];
        let equals_sign = split[1];
        let value = split[2];

        if equals_sign != "=" {
            bail!("expected equals sign");
        }

        match target {
            "mask" => {
                // we can use chars & len here because everything is ascii
                for (ch, place) in value.chars().zip((0..value.len()).rev()) {
                    self.mask[place] = match ch {
                        'X' => None,
                        '1' => Some(true),
                        '0' => Some(false),
                        oh_no => bail!("unknown char {} in mask", oh_no),
                    }
                }
            }
            memory => {
                let caps = MEM_REGEX
                    .captures(memory)
                    .ok_or_else(|| anyhow!("memory regex failed to match on {}", memory))?;
                let addr_str = caps
                    .get(1)
                    .ok_or_else(|| anyhow!("memory regex didn't have group 1 on {}", memory))?;

                // without the explicit type, rustc thinks it is a unit
                // :thonk:
                let addr: u64 = addr_str.as_str().parse()?;
                let value: u64 = value.parse()?;

                // The address we're writing to, sans floats.
                let unfloating_addr =
                    self.mask.iter().enumerate().fold(addr, |addr, (idx, bit)| {
                        if let Some(true) = bit {
                            // add the certainly 1 mask bit to the address
                            addr | (1 << idx)
                        } else {
                            // pass it through unchanged
                            addr
                        }
                    });

                let float_count = self.mask.iter().filter(|bit| bit.is_none()).count();
                for mask_modifier in 0..2u64.pow(float_count as u32) {
                    let mut mask_modifier = mask_modifier;
                    let mut addr = unfloating_addr;

                    for (idx, bit) in self.mask.iter().enumerate() {
                        if bit.is_none() {
                            // Pop off the least significant bit of the modifier and put it on
                            let bit = mask_modifier & 0b1;
                            // shift off the bit
                            // man these ligatures are weird
                            mask_modifier >>= 1;
                            // and assign this bit
                            let bit_shifted = 1 << idx;
                            if bit == 0 {
                                // clear bit
                                addr &= !bit_shifted;
                            } else {
                                // set the bit
                                addr |= bit_shifted;
                            }
                        }
                    }
                    // and assign the address!
                    self.memory.insert(addr, value);
                }
            }
        }

        Ok(())
    }
}

#[test]
fn part1() -> Result<()> {
    let mut program = ProgramP1::new();
    for line in INPUT.lines() {
        program.input(line)?;
    }
    // Get sum
    let sum: u64 = program.memory.values().sum();
    println!("sum: {}", sum);

    Ok(())

    // 13484360637149: too high
}

#[test]
fn part2() -> Result<()> {
    let mut program = ProgramP2::new();
    for line in INPUT.lines() {
        program.input(line)?;
    }
    // Get sum
    let sum: u64 = program.memory.values().sum();
    println!("sum: {}", sum);

    Ok(())

    // 1804269557402: too low
}

#[test]
fn part1_test() -> Result<()> {
    let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let mut program = ProgramP1::new();
    for line in input.lines() {
        program.input(line)?;
    }
    // Get sum
    let sum: u64 = program.memory.values().sum();
    println!("sum: {}", sum);

    Ok(())
}

#[test]
fn part2_test() -> Result<()> {
    let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    let mut program = ProgramP2::new();
    for line in input.lines() {
        program.input(line)?;
    }
    // Get sum
    let sum: u64 = program.memory.values().sum();
    println!("sum: {}", sum);

    Ok(())
}
