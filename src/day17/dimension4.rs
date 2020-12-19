use anyhow::{bail, Result};

use std::collections::{HashMap, HashSet};

pub struct Dimension {
    /// A set of all the active cubes
    world: HashSet<Coord4>,
    /// Current time
    time: u32,
}

impl Dimension {
    /// Create a new dimension from the input.
    /// The upper-left of the input is at (0, 0, 0).
    /// X increases to the right, Y down.
    pub fn new(i: &str) -> Result<Self> {
        let mut world = HashSet::new();
        for (y, line) in i.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                let active = match chr {
                    '#' => true,
                    '.' => false,
                    oh_no => bail!("unknown char `{}` at ({}, {}).", oh_no, x, y),
                };
                if active {
                    world.insert(Coord4::new(x as i32, y as i32, 0, 0));
                }
            }
        }

        Ok(Self { world, time: 0 })
    }

    /// Step once.
    pub fn step(&mut self) {
        let mut newmap = HashSet::new();

        // First consider all the active cubes and see if they stay hot.
        // Also record the min and max X, Y, and Z we see.
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut min_z = 0;
        let mut max_z = 0;
        let mut min_w = 0;
        let mut max_w = 0;
        for &active in self.world.iter() {
            // update bounds
            min_x = min_x.min(active.x);
            max_x = max_x.max(active.x);
            min_y = min_y.min(active.y);
            max_y = max_y.max(active.y);
            min_z = min_z.min(active.z);
            max_z = max_z.max(active.z);
            min_w = min_w.min(active.w);
            max_w = max_w.max(active.w);

            let neighbor_count = active
                .neighbors()
                .filter(|n| self.world.contains(&n))
                .count();
            if neighbor_count == 2 || neighbor_count == 3 {
                // activate it!
                newmap.insert(active);
            } // Else, we don't put it in there and it stays deactivated.
        }

        // Now consider all the un-active cubes
        // We inflate the bounds by one in each direction.
        // Imagine the world right now is just the XY-plane of cubes
        // We would want to go to z = -1 and 1 to check, but no further
        // as that would be redundant.
        for x in min_x - 1..=max_x + 1 {
            for y in min_y - 1..=max_y + 1 {
                for z in min_z - 1..=max_z + 1 {
                    for w in min_w - 1..=max_w + 1 {
                        let coord = Coord4::new(x, y, z, w);
                        if !self.world.contains(&coord) {
                            // ok this is a dead cube! will it wake up?
                            let neighbor_count = coord
                                .neighbors()
                                .filter(|n| self.world.contains(&n))
                                .count();
                            if neighbor_count == 3 {
                                // it is here!
                                newmap.insert(coord);
                            } // Else we don't insert it
                        }
                    }
                }
            }
        }

        self.world = newmap;
        self.time += 1;
    }

    /// Get a reference to the world.
    pub fn get_world(&self) -> &HashSet<Coord4> {
        &self.world
    }
}

/// a 3-D coordinate
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coord4 {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }

    /// Get a list of this cube's neighbors
    pub fn neighbors(self) -> NeighborsIter {
        NeighborsIter {
            original: self,
            time: 0,
        }
    }
}

/// Iterator for a Coord4's neighbors. Iteration order is undefined.
pub struct NeighborsIter {
    original: Coord4,
    time: i32,
}

impl Iterator for NeighborsIter {
    type Item = Coord4;
    fn next(&mut self) -> Option<Self::Item> {
        if self.time >= 3i32.pow(4) {
            // no more!
            return None;
        }
        let dx = self.time % 3 - 1;
        let dy = (self.time / 3) % 3 - 1;
        let dz = (self.time / 3 / 3) % 3 - 1;
        let dw = (self.time / 3 / 3 / 3) % 3 - 1;
        self.time += 1;

        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
            // nope, won't return the original
            self.next()
        } else {
            // return these offsets
            Some(Coord4 {
                x: self.original.x + dx,
                y: self.original.y + dy,
                z: self.original.z + dz,
                w: self.original.w + dw,
            })
        }
    }
}

#[test]
fn test_neighbors() {
    let coord = Coord4::new(0, 0, 0, 0);
    for n in coord.neighbors() {
        println!("{:?}", n);
    }
    println!("{} locations", coord.neighbors().count());
}
