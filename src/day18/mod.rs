mod parsing;

use parsing::Term;

use anyhow::{anyhow, Context, Result};

const INPUT: &str = include_str!("input.txt");

/// Parse a string into a bunch of terms line-by-line.
fn parse_lines_communist(input: &str) -> Result<Vec<Term>> {
    input
        .lines()
        .map(|line| Term::lex_parse_communist(line))
        .collect()
}

fn parse_lines_precedentful(input: &str) -> Result<Vec<Term>> {
    input
        .lines()
        .map(|line| {
            Term::lex_parse_precedentful(line).with_context(|| anyhow!("when lex/parsing {}", line))
        })
        .collect()
}

#[test]
fn part1() -> Result<()> {
    let terms = parse_lines_communist(INPUT)?;
    let values = terms
        .iter()
        .map(|term| term.evaluate())
        .collect::<Result<Vec<_>>>()?;
    let sum: u64 = values.iter().sum();
    println!("sum: {}", sum);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let terms = parse_lines_precedentful(INPUT)?;
    let values = terms
        .iter()
        .map(|term| term.evaluate())
        .collect::<Result<Vec<_>>>()?;
    let sum: u64 = values.iter().sum();
    println!("sum: {}", sum);

    Ok(())

    // 734848159905949: too high
}

#[test]
fn parse_test() -> Result<()> {
    let input = r"1 + 2 * 3 + 4 * 5 + 6
2 * 3 + (4 * 5)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
    let terms = parse_lines_communist(input)?;
    for (term, line) in terms.iter().zip(input.lines()) {
        println!("{} -> {}", line, term.pretty_print());
    }

    Ok(())
}

#[test]
fn parse_capitalist() -> Result<()> {
    let input = r"2 * 3 + 4 + 5 * 6
1 + 2 * 3 + 4 * 5 + 6
2 * 3 + (4 * 5)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
    let terms = parse_lines_precedentful(input)?;
    for (term, line) in terms.iter().zip(input.lines()) {
        println!("{}   ->   {}", line, term.pretty_print());
    }

    Ok(())
}

#[test]
fn parse_capitalist1() -> Result<()> {
    let input = "3 * (4 + 5) * 6";
    let term = Term::lex_parse_precedentful(input)?;
    println!("{}\n{}\n{:#?}", input, term.pretty_print(), term);

    Ok(())
}
