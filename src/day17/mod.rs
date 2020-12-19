mod dimension3;
mod dimension4;

use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

#[test]
fn part1() -> Result<()> {
    let mut dimension = dimension3::Dimension::new(INPUT)?;

    for _ in 0..6 {
        dimension.step();
    }

    let active = dimension.get_world().len();
    println!("active cubes: {}", active);

    Ok(())

    // 245: too low
}

#[test]
fn part2() -> Result<()> {
    // this function looks familiar
    let mut dimension = dimension4::Dimension::new(INPUT)?;

    for _ in 0..6 {
        dimension.step();
    }

    let active = dimension.get_world().len();
    println!("active cubes: {}", active);

    Ok(())

    // 4029: too high
}
