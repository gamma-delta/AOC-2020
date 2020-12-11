use std::{
    convert::TryInto,
    io::{stdout, Write},
    iter::{self, repeat},
};

use anyhow::{bail, Result};
use fwdansi::write_ansi;
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor};

const INPUT: &str = include_str!("input.txt");

const SMOL_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

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

    /// Update one step using part 1's rules. Return if there were any changes.
    fn update_part1(&mut self) -> bool {
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
                    // no out of bounds >:(
                    if check_x >= self.width || check_y >= self.height {
                        return None;
                    }
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

    /// Update one step using part 2's rules.
    /// Return if there were any changes
    fn update_part2(&mut self) -> bool {
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
                .filter(|&(dx, dy)| {
                    // Check if anything is down that way
                    // there's gotta be a better way to do this
                    let x_iter: Box<dyn Iterator<Item = usize>> = match dx {
                        -1 => Box::new((0..x).rev()),
                        0 => Box::new(iter::repeat(x)),
                        1 => Box::new((x + 1)..self.width),
                        _ => unreachable!(),
                    };
                    let y_iter: Box<dyn Iterator<Item = usize>> = match dy {
                        -1 => Box::new((0..y).rev()),
                        0 => Box::new(iter::repeat(y)),
                        1 => Box::new((y + 1)..self.height),
                        _ => unreachable!(),
                    };
                    // find the first not-floor spot
                    // and check if it's a full chair
                    x_iter.zip(y_iter).find_map(|(check_x, check_y)| {
                        let spot_there = *self.spots.get(check_y * self.width + check_x).unwrap();
                        match spot_there {
                            Spot::Floor => None,
                            Spot::EmptyChair => Some(false),
                            Spot::FullChair => Some(true),
                        }
                    }) == Some(true)
                })
                .count();
                if *seat == Spot::EmptyChair && filled_seats == 0 {
                    any_change = true;
                    *seat = Spot::FullChair;
                } else if *seat == Spot::FullChair && filled_seats >= 5 {
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
        // write_ansi(&mut stdout, b"\x1b[2J\x1b[H")?;

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
    while ferry.update_part1() {
        // ferry.print()?;
        // println!();
    }

    let occupied = ferry
        .spots
        .iter()
        .filter(|spot| **spot == Spot::FullChair)
        .count();
    println!("seats occupied: {}", occupied);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let mut ferry = Ferry::new(INPUT)?;

    // what is this, c?
    // let mut idx = 0;
    while ferry.update_part2() {
        // idx += 1;
        // println!("step {}:", idx); // cover yourself in oil
        // ferry.print()?;
        // println!();
    }

    let occupied = ferry
        .spots
        .iter()
        .filter(|spot| **spot == Spot::FullChair)
        .count();
    println!("seats occupied: {}", occupied);

    Ok(())

    // not 4
}
