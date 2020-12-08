mod parsing;

use anyhow::Result;

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

/// no mating
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Pattern {
    pub quality: String,
    pub color: String,
}

struct BagCollection {
    /// Maps bag patterns to the counts and patterns it can contain
    bags: HashMap<Pattern, Vec<(usize, Pattern)>>,
}

impl BagCollection {
    /// Search for the number of ways to contain the given pattern
    fn search(&self, target: &Pattern) -> usize {
        let bags = &self.bags;

        // Let's do some memoization
        // Maps bag patterns to if we've ever seen the target inside
        let mut cache = HashMap::new();

        fn search_inner(
            target: &Pattern,
            search_location: &Pattern,
            bags: &HashMap<Pattern, Vec<(usize, Pattern)>>,
            cache: &mut HashMap<Pattern, bool>,
        ) -> bool {
            if let Some(&found) = cache.get(search_location) {
                // we already know about this one
                return found;
            }

            let contained = &bags[search_location];
            let anywhere_inside = contained
                .iter()
                // is it *any*where inside?
                .any(|(_count, pattern)| {
                    if pattern == target {
                        // we found it!
                        true
                    } else {
                        // hm maybe someone else knows something...
                        search_inner(target, pattern, bags, cache)
                    }
                });

            // Memoize it
            cache.insert(search_location.clone(), anywhere_inside);
            anywhere_inside
        }

        bags.keys()
            .filter(|pat| search_inner(target, pat, &bags, &mut cache))
            .count()
    }

    /// Count the number of bags inside the given bag
    fn count(&self, target: &Pattern) -> usize {
        let bags = &self.bags;

        // Let's do some memoization
        // Maps bag patterns to the number of bags inside
        let mut cache = HashMap::new();

        fn count_inner(
            target: &Pattern,
            bags: &HashMap<Pattern, Vec<(usize, Pattern)>>,
            cache: &mut HashMap<Pattern, usize>,
        ) -> usize {
            if let Some(&found) = cache.get(target) {
                // we already know about this one
                return found;
            }

            let contained = &bags[target];
            let count = contained
                .iter()
                // count the bags, plus the bags inside each bag
                .map(|(count, pattern)| count + count * count_inner(pattern, bags, cache))
                .sum();

            // Memoize it
            cache.insert(target.clone(), count);
            count
        }

        count_inner(target, bags, &mut cache)
    }
}

#[test]
fn part1() -> Result<()> {
    let bags = BagCollection::parse_input(INPUT)?;
    let count = bags.search(&Pattern {
        quality: "shiny".to_string(),
        color: "gold".to_string(),
    });
    println!("count: {}", count);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let bags = BagCollection::parse_input(INPUT)?;
    let count = bags.count(&Pattern {
        quality: "shiny".to_string(),
        color: "gold".to_string(),
    });
    println!("count: {}", count);

    Ok(())
}

#[test]
fn part1_test() -> Result<()> {
    let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let bags = BagCollection::parse_input(input)?;
    let count = bags.search(&Pattern {
        quality: "shiny".to_string(),
        color: "gold".to_string(),
    });
    println!("count: {}", count);

    Ok(())
}
