const INPUT: &str = include_str!("input.txt");

/// A forest full of trees.
/// Internally only stores what is in the puzzle input; indexing it
/// gets the proper trees if they're out of bounds.
struct Forest {
    /// indexed `[y][x]`.
    ///
    /// `true` = a tree is here
    trees: Vec<Vec<bool>>,
    /// Width of the forest disregarding looping
    width: usize,
}

impl Forest {
    /// Make a new Forest from the input
    fn new(input: &str) -> Self {
        let mut width = None;
        let trees = input
            .lines()
            .map(|line| {
                if width.is_none() {
                    width = Some(line.len())
                } else {
                    assert_eq!(
                        width,
                        Some(line.len()),
                        "A line was the wrong length! Expected {:?}",
                        width
                    );
                }

                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        oh_no => panic!("the character {} was encountered!", oh_no),
                    })
                    .collect()
            })
            .collect();

        Self {
            trees,
            width: width.unwrap(),
        }
    }

    /// Is there a tree at the given coordinate?
    /// Return None if it is out-of-bounds to the south
    fn tree_at(&self, x: usize, y: usize) -> Option<bool> {
        let x = x % self.width;
        self.trees.get(y).map(|row| row.get(x).copied()).flatten()
    }

    /// Traverse the wilderness with the given dx and dy.
    /// Return the number of trees hit.
    fn traverse(&self, dx: usize, dy: usize) -> usize {
        let mut count = 0;
        for step in 0.. {
            let x = step * dx;
            let y = step * dy;
            match self.tree_at(x, y) {
                Some(true) => count += 1,
                Some(false) => {} // do nothing
                None => break,
            }
        }
        count
    }
}

#[test]
fn part1() {
    let forest = Forest::new(INPUT);
    let count = forest.traverse(3, 1);
    println!("Trees hit: {}", count);
}

#[test]
fn part2() {
    let forest = Forest::new(INPUT);
    let mega_count = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(dx, dy)| forest.traverse(dx, dy))
        .fold_first(|acc, x| acc * x)
        .unwrap();
    println!("MEGA COUNT: {}", mega_count);
}
