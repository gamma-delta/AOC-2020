use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Seat {
    row: i16,
    column: i16,
}

impl Seat {
    /// How many rows are there?
    const ROWS: i16 = 128;
    /// How manu columns are there?
    const COLUMNS: i16 = 8;

    /// Turn an input string into a Seat.
    /// yes this should be from_str whatever
    fn parse(i: &str) -> Seat {
        let col_start_idx = i.find(|c: char| c == 'L' || c == 'R').unwrap();
        let (row, col) = i.split_at(col_start_idx);
        let row = partition(row, 'F', 'B');
        let column = partition(col, 'L', 'R');
        Seat { row, column }
    }

    /// Get the seat ID of this.
    fn seat_id(&self) -> i16 {
        self.row * 8 + self.column
    }
}

/// Partition into a value
fn partition(input: &str, take_low: char, take_high: char) -> i16 {
    input
        .chars()
        .take_while(|&c| c == take_low || c == take_high)
        .fold(0, |acc, c| acc * 2 + (c == take_high) as i16)
}

#[test]
fn part1() {
    let seats = INPUT
        .lines()
        .map(|line| Seat::parse(line))
        .collect::<Vec<_>>();

    let max_seatid = seats.iter().map(|seat| seat.seat_id()).max().unwrap();
    println!("max seat id: {}", max_seatid);
}

#[test]
fn part2() {
    let seats = INPUT
        .lines()
        .map(|line| Seat::parse(line))
        .collect::<Vec<_>>();
    let ids_present = seats
        .iter()
        .map(|seat| seat.seat_id())
        .collect::<HashSet<_>>();

    let max_id = Seat {
        row: Seat::ROWS - 1,
        column: Seat::COLUMNS - 1,
    }
    .seat_id();
    for id in 0..=max_id {
        if !ids_present.contains(&id)
            && ids_present.contains(&(id - 1))
            && ids_present.contains(&(id + 1))
        {
            // we got it!
            println!("my seat ID is {}", id);
            break;
        }
    }
}
