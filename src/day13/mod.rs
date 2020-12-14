use anyhow::{anyhow, bail, Result};

const INPUT: &str = include_str!("input.txt");

struct Timetable {
    /// When I need to leave
    depart: u32,
    /// IDs of the buses, or None if it's X
    buses: Vec<Option<u32>>,
}

impl Timetable {
    /// Make a new timetable from the input.
    fn new(input: &str) -> Result<Self> {
        let mut lines = input.lines();
        let depart = lines
            .next()
            .ok_or_else(|| anyhow!("not enough lines"))?
            .parse()?;
        let buses = lines.next().ok_or_else(|| anyhow!("not enough lines"))?;
        let buses = buses
            .split(',')
            .map(|entry| {
                if entry == "x" {
                    Ok(None)
                } else {
                    Ok(Some(entry.parse()?))
                }
            })
            .collect::<Result<_>>()?;

        Ok(Self { depart, buses })
    }

    /// Find the `(id, dtime)` of when you can leave
    fn leave(&self) -> (u32, u32) {
        for time in self.depart.. {
            for bus in self.buses.iter() {
                if let Some(bus) = bus {
                    if time % bus == 0 {
                        // poggers
                        let dt = time - self.depart;
                        return (*bus, dt);
                    }
                }
            }
        }

        panic!("oh no there was never a bus ;-;")
    }

    /// Find the earliest timestamp the buses will leave sequentially in
    fn earliest_sequence(&self, start_at: u64) -> u64 {
        let mut time = start_at;
        let mut stepsize = 1;
        let mut check_count = 1;
        loop {
            let success = self
                .buses
                .iter()
                .take(check_count)
                .enumerate()
                .all(|(idx, bus)| {
                    if let Some(bus) = bus {
                        (time + idx as u64) % *bus as u64 == 0
                    } else {
                        // if there's no requirement there, it's always OK
                        true
                    }
                });
            if success {
                // the first check_count busses check out.
                if check_count == self.buses.len() {
                    // eyy we're good!
                    return time;
                }

                // Ordinarily we would tak the LCM, but because all the bus IDs are prime we can just multiply.
                let new_step_size =
                    self.buses
                        .iter()
                        .take(check_count)
                        .fold(1, |acc, bus| match bus {
                            Some(id) => acc * *id as u64,
                            None => acc,
                        });
                stepsize = new_step_size;
                check_count += 1;

                // Make sure we're not skipping a solution
                // apparently
                // i have no idea what this does but alcatraz promises me it's important
                if time > new_step_size {
                    time %= new_step_size;
                }
            }

            time += stepsize;
        }
    }
}

#[test]
fn part1() -> Result<()> {
    let timetable = Timetable::new(INPUT)?;
    let (id, dt) = timetable.leave();
    println!("bus #{} with {}m waiting = {}", id, dt, id * dt);
    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let timetable = Timetable::new(INPUT)?;
    // they told us where we're starting
    let time = timetable.earliest_sequence(100000000000000);
    println!("earliest time: {}", time);
    Ok(())

    // 1602195830254321 is too high
}

#[test]
fn part2_test() -> Result<()> {
    let input = "69420\n67,7,59,61";
    let timetable = Timetable::new(input)?;
    let time = timetable.earliest_sequence(0);
    println!("earliest time: {}", time);
    Ok(())
}
