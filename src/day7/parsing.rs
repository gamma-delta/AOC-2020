use super::{BagCollection, Pattern};

use anyhow::{anyhow, bail, Result};

impl BagCollection {
    /// Parse an input into the bags.
    pub fn parse_input(input: &str) -> Result<Self> {
        let bags = input
            .lines()
            .enumerate()
            .map(|(num, line)| parse_line(line, num).map(|bag| (bag.pattern, bag.contains)))
            .collect::<Result<_>>()?;
        Ok(Self { bags })
    }
}

struct Bag {
    pattern: Pattern,
    contains: Vec<(usize, Pattern)>,
}

/// Parse a single line
fn parse_line(line: &str, line_num: usize) -> Result<Bag> {
    // lines start at 1
    let line_num = line_num + 1;

    let split = line.split_ascii_whitespace().collect::<Vec<_>>();

    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    let (pattern, i) = parse_pattern(&split, line_num)?;
    // contain 1 bright white bag, 2 muted yellow bags.
    let (must_contain, i) = i
        .split_first()
        .ok_or_else(|| anyhow!("parse_line: line {}: eof at must_contain", line_num))?;
    if !must_contain.starts_with("contain") {
        bail!("expected `contain*` but got `{}`", must_contain);
    }
    // 1 bright white bag, 2 muted yellow bags.
    // OR
    // no other bags.
    if let Some(&"no") = i.get(0) {
        // No other bags!
        return Ok(Bag {
            pattern,
            contains: Vec::new(),
        });
    }
    // 1 bright white bag, 2 muted yellow bags.
    let contains: Vec<_> = i
        .chunks_exact(4)
        .map(|chunk| {
            // 1 bright white bag,
            let (count, i) = chunk
                .split_first()
                .ok_or_else(|| anyhow!("parse_line: line {}: eof at count", line_num))?;
            // bright white bag,
            let count = count.parse()?;
            let (pattern, i) = parse_pattern(i, line_num)?;
            if !i.is_empty() {
                bail!("parse_line: line {}: not empty after chunks", line_num);
            }
            Ok((count, pattern))
        })
        .collect::<Result<_>>()?;

    Ok(Bag { pattern, contains })
}

/// Parse a pattern.
///
/// `<quality> <color> bag(s)`
fn parse_pattern<'a>(i: &'a [&str], line_num: usize) -> Result<(Pattern, &'a [&'a str])> {
    let (quality, i) = i
        .split_first()
        .ok_or_else(|| anyhow!("parse_pattern: line {}: eof at quality", line_num))?;
    let (color, i) = i
        .split_first()
        .ok_or_else(|| anyhow!("parse_pattern: line {}: eof at color", line_num))?;
    let (must_bag, i) = i
        .split_first()
        .ok_or_else(|| anyhow!("parse_pattern: line {}: eof at must_bag", line_num))?;
    if !must_bag.starts_with("bag") {
        bail!("expected `bag*` but got `{}`", must_bag);
    }
    Ok((
        Pattern {
            quality: quality.to_string(),
            color: color.to_string(),
        },
        i,
    ))
}
