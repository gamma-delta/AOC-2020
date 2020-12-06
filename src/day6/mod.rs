use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[test]
fn part1() {
    let groups = INPUT.split("\n\n");
    let group_sets = groups
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold_first(|acc, set| acc.union(&set).cloned().collect())
                .unwrap()
        })
        .collect::<Vec<_>>();
    let total_count: usize = group_sets.iter().map(|set| set.len()).sum();
    println!("total count: {}", total_count);
}

#[test]
fn part2() {
    let groups = INPUT.split("\n\n");
    let group_sets = groups
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold_first(|acc, set| acc.intersection(&set).cloned().collect())
                .unwrap()
        })
        .collect::<Vec<_>>();
    let total_count: usize = group_sets.iter().map(|set| set.len()).sum();
    println!("total intersection count: {}", total_count);
}
