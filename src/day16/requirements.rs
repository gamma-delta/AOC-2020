use anyhow::{anyhow, bail, Result};

use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Requirement {
    /// Each requirement has two ranges in it; here's the first
    req1: RangeInclusive<u32>,
    /// and here's the second
    req2: RangeInclusive<u32>,
}

impl Requirement {
    /// Make a new Requirement from a slice of words
    pub fn new<'slice>(words: &'slice [&'slice str]) -> Result<(Self, &'slice [&'slice str])> {
        let (req1, words) = words
            .split_first()
            .ok_or_else(|| anyhow!("could not split req1"))?;
        let (must_or, words) = words
            .split_first()
            .ok_or_else(|| anyhow!("could not split or"))?;
        if *must_or != "or" {
            bail!("expected `or`, got `{}`", must_or);
        }
        let (req2, words) = words
            .split_first()
            .ok_or_else(|| anyhow!("could not split req2"))?;

        let req1 = to_range(req1)?;
        let req2 = to_range(req2)?;
        Ok((Self { req1, req2 }, words))
    }

    /// Check if a number satisfies the requirement
    pub fn satisfied_by(&self, num: u32) -> bool {
        self.req1.contains(&num) || self.req2.contains(&num)
    }
}

/// Convert a string like `123-456` into a Range
fn to_range(i: &str) -> Result<RangeInclusive<u32>> {
    let split = i.split('-').collect::<Vec<_>>();
    match split.as_slice() {
        [lo, hi] => {
            let lo = lo.parse()?;
            let hi = hi.parse()?;
            Ok(lo..=hi)
        }
        _ => bail!("could not split `{}` properly", i),
    }
}
