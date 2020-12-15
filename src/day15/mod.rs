use std::collections::HashMap;

const INPUT: &str = "13,0,10,12,1,5,8";

#[derive(Debug)]
struct GameIter<'s> {
    /// The input numbers
    input: &'s [usize],
    /// The current time, counting upwards
    now: usize,
    /// Maps numbers to the time they were last said
    spoken: HashMap<usize, usize>,
    /// The last spoken number.
    last_spoken: Option<usize>,
}

impl<'s> GameIter<'s> {
    /// Make a new GameIter from the input.
    fn new(input: &'s [usize]) -> Self {
        Self {
            input,
            now: 0,
            spoken: HashMap::new(),
            // we can ignore this value because we're going to overwrite it
            last_spoken: None,
        }
    }
}

impl<'s> Iterator for GameIter<'s> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // what are we going to say?
        let speaks = if let Some(starting_num) = self.input.get(self.now) {
            // we're in the setup phase, say one of the numbers
            *starting_num
        } else {
            // out of the woods now.
            // what info do we have on when we last said a number?
            // unwrap is ok because we filled it
            if let Some(&last_said_time) = self.spoken.get(&self.last_spoken.unwrap()) {
                // how many turns farther are we?
                self.now - last_said_time
            } else {
                // we've never said this before, so return 0 as per the instructions.
                0
            }
        };
        // Update the info
        // we insert the *previously* spoken thing
        if let Some(last_spoken) = self.last_spoken {
            self.spoken.insert(last_spoken, self.now);
        }
        self.last_spoken = Some(speaks);
        // Update the time
        self.now += 1;
        // And return the thing spoken
        Some(speaks)
    }
}

#[test]
fn part1() {
    let input = INPUT
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    println!("2020th: {:?}", GameIter::new(&input).nth(2020 - 1));

    // not 1
}

#[test]
fn part2() {
    let input = INPUT
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    println!("30000000th: {:?}", GameIter::new(&input).nth(30000000 - 1));
}

#[test]
fn part1_test() {
    let input = "0,3,6"
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();

    for (idx, num) in GameIter::new(&input).take(10).enumerate() {
        println!("#{} -> {}", idx + 1, num);
    }
}
