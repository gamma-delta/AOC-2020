use std::{
    convert::TryInto,
    io::{stdout, Write},
};

use anyhow::{bail, Result};
use fwdansi::write_ansi;
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spot {
    Floor,
    EmptyChair,
    FullChair,
}

struct Ferry {
    width: usize,
    height: usize,
    /// Spots that passengers sit in.
    /// `y * width + x`
    spots: Vec<Spot>,
}

impl Ferry {
    /// Make a new Ferry from the input
    fn new(i: &str) -> Result<Ferry> {
        let mut width = None;
        let lines = i.lines();
        let spots = lines
            .flat_map(|line| {
                width.get_or_insert_with(|| line.chars().count());
                line.chars().map(|c| {
                    Ok(match c {
                        '.' => Spot::Floor,
                        'L' => Spot::EmptyChair,
                        '#' => Spot::FullChair,
                        oh_no => bail!("unknown character `{}`", oh_no),
                    })
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let height = i.lines().count();

        Ok(Ferry {
            width: width.unwrap(),
            height,
            spots,
        })
    }

    /// Update one step. Return if there were any changes.
    fn update(&mut self) -> bool {
        let mut spots = self.spots.clone();
        let mut any_change = false;

        for x in 0..self.width {
            'y: for y in 0..self.height {
                let seat = spots.get_mut(y * self.width + x).unwrap();
                if *seat == Spot::Floor {
                    continue 'y;
                }
                let filled_seats = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                ]
                .iter()
                .filter_map(|&(dx, dy)| {
                    let check_x: usize = (x as i32 + dx).try_into().ok()?;
                    let check_y: usize = (y as i32 + dy).try_into().ok()?;
                    self.spots
                        .get(check_y * self.width + check_x)
                        .and_then(|&spot| {
                            if spot == Spot::FullChair {
                                Some(())
                            } else {
                                None
                            }
                        })
                })
                .count();
                if *seat == Spot::EmptyChair && filled_seats == 0 {
                    any_change = true;
                    *seat = Spot::FullChair;
                } else if *seat == Spot::FullChair && filled_seats >= 4 {
                    any_change = true;
                    *seat = Spot::EmptyChair;
                }
            }
        }

        self.spots = spots;
        any_change
    }

    /// Print this to the console
    fn print(&self) -> Result<()> {
        let mut stdout = termcolor::StandardStream::stdout(ColorChoice::Always);
        write_ansi(&mut stdout, b"\x1b[2J\x1b[H")?;

        let mut col = ColorSpec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let (ch, _) = match self.spots[y * self.width + x] {
                    Spot::Floor => (b".", col.set_fg(Some(Color::Green))),
                    Spot::EmptyChair => (b"L", col.set_fg(Some(Color::Blue))),
                    Spot::FullChair => (b"#", col.set_fg(Some(Color::Cyan))),
                };
                stdout.set_color(&col)?;
                stdout.write_all(ch)?;
            }
            stdout.write_all(b"\n")?;
        }

        Ok(())
    }
}

#[test]
fn part1() -> Result<()> {
    let mut ferry = Ferry::new(INPUT)?;

    // what is this, c?
    while ferry.update() {}

    let occupied = ferry
        .spots
        .iter()
        .filter(|spot| **spot == Spot::FullChair)
        .count();
    println!("seats occupied: {}", occupied);

    Ok(())

    // not 0
    // 2223 is too low
}
