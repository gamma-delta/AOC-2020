use once_cell::sync::Lazy;

static INPUT: Lazy<Vec<u32>> = Lazy::new(|| {
    include_str!("input.txt")
        .split('\n')
        .filter_map(|it| it.parse().ok())
        .collect()
});

#[test]
fn part1() {
    for (idx1, &val1) in INPUT.iter().enumerate() {
        for (idx2, &val2) in INPUT.iter().enumerate() {
            if idx1 != idx2 && val1 + val2 == 2020 {
                // we got it!
                println!("{0} + {1} = 2020; {0} * {1} = {2}", val1, val2, val1 * val2);
            }
        }
    }
}

#[test]
fn part2() {
    for (idx1, &val1) in INPUT.iter().enumerate() {
        for (idx2, &val2) in INPUT.iter().enumerate() {
            for (idx3, &val3) in INPUT.iter().enumerate() {
                if idx1 != idx2 && idx2 != idx3 && idx1 != idx3 && val1 + val2 + val3 == 2020 {
                    // we got it!
                    println!("{} * {} * {} = {}", val1, val2, val3, val1 * val2 * val3);
                }
            }
        }
    }
}
