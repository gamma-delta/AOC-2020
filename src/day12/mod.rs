use anyhow::{bail, Result};
use cogs_gamedev::{directions::Direction4, int_coords::ICoord};

const INPUT: &str = include_str!("input.txt");

struct Ferry {
    position: ICoord,
    heading: Direction4,
}

impl Ferry {
    /// Make a new Feryy with default position and direction
    fn new() -> Self {
        Self {
            position: ICoord::new(0, 0),
            heading: Direction4::East,
        }
    }

    /// Accept one action
    fn act_on(&mut self, i: &str) -> Result<()> {
        let (cmd, amount) = i.split_at(1);
        let amount: isize = amount.parse()?;
        match cmd {
            "N" => self.position += ICoord::new(0, -amount),
            "S" => self.position += ICoord::new(0, amount),
            "E" => self.position += ICoord::new(amount, 0),
            "W" => self.position += ICoord::new(-amount, 0),
            "L" | "R" => {
                // "backwards" trig because Y is down
                let rot = amount / 90 * if cmd == "L" { -1 } else { 1 };
                let newdir = self.heading.rotate(rot);
                self.heading = newdir;
            }
            "F" => self.position += self.heading.deltas() * amount,
            oh_no => bail!("unknown command `{}`", oh_no),
        }

        Ok(())
    }

    /// Get the manhattan distance from the origin.
    fn distance(&self) -> isize {
        self.position.x.abs() + self.position.y.abs()
    }
}

struct FerryWithWaypoint {
    position: ICoord,
    /// Delta from the ferry to the waypoint
    dwaypoint: ICoord,
}

impl FerryWithWaypoint {
    fn new() -> Self {
        Self {
            position: ICoord::new(0, 0),
            dwaypoint: ICoord::new(10, -1),
        }
    }

    /// Accept one action
    fn act_on(&mut self, i: &str) -> Result<()> {
        let (cmd, amount) = i.split_at(1);
        let amount: isize = amount.parse()?;
        match cmd {
            "N" => self.dwaypoint += ICoord::new(0, -amount),
            "S" => self.dwaypoint += ICoord::new(0, amount),
            "E" => self.dwaypoint += ICoord::new(amount, 0),
            "W" => self.dwaypoint += ICoord::new(-amount, 0),
            "L" | "R" => {
                let rot = amount / 90 * if cmd == "L" { -1 } else { 1 };
                // x, y ->
                // 0: x, y
                // 1: -y, x
                // 2: -x, -y
                // 3: y, -x
                let (x, y) = if rot.rem_euclid(2) == 0 {
                    (self.dwaypoint.x, self.dwaypoint.y)
                } else {
                    (self.dwaypoint.y, self.dwaypoint.x)
                };
                let xneg = if (rot - 1).rem_euclid(4) <= 1 { -1 } else { 1 };
                let yneg = if rot.rem_euclid(4) >= 2 { -1 } else { 1 };
                self.dwaypoint = ICoord::new(x * xneg, y * yneg);
            }
            "F" => self.position += self.dwaypoint * amount,
            oh_no => bail!("unknown command `{}`", oh_no),
        }

        Ok(())
    }

    /// Get the manhattan distance from the origin.
    fn distance(&self) -> isize {
        self.position.x.abs() + self.position.y.abs()
    }
}

#[test]
fn part1() -> Result<()> {
    let mut ferry = Ferry::new();
    for line in INPUT.lines() {
        ferry.act_on(line)?;
    }
    println!("distance gone: {}", ferry.distance());

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let mut ferry = FerryWithWaypoint::new();
    for line in INPUT.lines() {
        ferry.act_on(line)?;
    }
    println!("distance gone: {}", ferry.distance());

    Ok(())

    // 129063 is too high
    // 11711 is too low
    // not 20212
    // not 18038
}
