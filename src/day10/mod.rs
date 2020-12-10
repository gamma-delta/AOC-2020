use anyhow::{bail, Result};

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

/// Given a set of adapters, find the number of jumps of each voltage
fn count_joltage_jumps(mut input: HashSet<u32>) -> Result<[u32; 3]> {
    // include the 3-jolt jump for the phone
    let mut out = [0, 0, 1];
    let mut current_joltage = 0;

    'adapters: while !input.is_empty() {
        for delta in 1..=3 {
            let jolts_out = current_joltage + delta;
            let used_adapter = input.remove(&jolts_out);
            if used_adapter {
                // nice we've plugged something else into our chain
                current_joltage = jolts_out;
                // record that
                out[(delta - 1) as usize] += 1;
                continue 'adapters;
            }
        }
        bail!(
            "when the joltage was {} there were no more valid adapters",
            current_joltage
        );
    }

    Ok(out)
}

/// Count the number of ways a set of chargers can go to the phone
fn count_joltage_perms(input: HashSet<u32>) -> u128 {
    // Inner function to count the number of things an adapter can get plugged into
    // The cache maps (number of jolts we are looking for) -> (number of ways things can accept that and still win)
    fn count_inner(
        jolts: u32,
        adapters: &HashSet<u32>,
        phone_jolts: u32,
        cache: &mut HashMap<u32, u128>,
    ) -> u128 {
        (1..=3)
            .map(|delta| {
                let looking_for = jolts + delta;
                if let Some(&ans) = cache.get(&looking_for) {
                    ans
                } else {
                    let ans = if looking_for == phone_jolts {
                        // we're at the end of the line!
                        // this permutation is OK
                        1
                    } else if adapters.contains(&looking_for) {
                        // let's keep going
                        count_inner(looking_for, adapters, phone_jolts, cache)
                    } else {
                        // this is invalid
                        0
                    };
                    cache.insert(looking_for, ans);
                    ans
                }
            })
            .sum()
    }

    let phone_jolts = input.iter().max().unwrap() + 3;
    let mut cache = HashMap::new();
    count_inner(0, &input, phone_jolts, &mut cache)
}

#[test]
fn part1() -> Result<()> {
    let input = INPUT
        .lines()
        .map(|line| Ok(line.parse::<u32>()?))
        .collect::<Result<_>>()?;
    let out = count_joltage_jumps(input)?;
    let solution = out[0] * out[2];
    println!("solution: {}", solution);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let input = INPUT
        .lines()
        .map(|line| Ok(line.parse::<u32>()?))
        .collect::<Result<_>>()?;
    let solution = count_joltage_perms(input);
    println!("solution: {}", solution);

    Ok(())
}
