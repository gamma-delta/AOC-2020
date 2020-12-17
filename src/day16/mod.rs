mod requirements;
mod parsing;

use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use parsing::Input;
use requirements::Requirement;

const INPUT: &str = include_str!("input.txt");

/// See if a ticket is valid given the requirements.
/// Return None if it's ok, or Some with the error rate if it's broken.
fn error_rate(ticket: &[u32], reqs: &HashMap<String, Requirement>) -> Option<u32> {
    let bads = ticket
        .iter()
        .filter(|num| !reqs.values().any(|req| req.satisfied_by(**num)))
        .cloned()
        .collect::<Vec<_>>();
    if bads.is_empty() {
        None
    } else {
        Some(bads.iter().sum())
    }
}

#[test]
fn part1() -> Result<()> {
    let input = Input::new(INPUT)?;
    let error_rate: u32 = input
        .other_tickets
        .iter()
        .filter_map(|ticket| error_rate(ticket, &input.requirements))
        .sum();
    println!("total error rate: {}", error_rate);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let input = Input::new(INPUT)?;
    let valid_tickets = input
        .other_tickets
        .iter()
        .filter(|ticket| {
            let erate = error_rate(ticket, &input.requirements);
            // if there was no error, keep the ticket
            // if there was an error, throw it away
            erate.is_none()
        })
        .collect::<Vec<_>>();

    // This maps each field index to the names of the field it might be
    let mut possible_fields = {
        let all_fields = input.requirements.keys().cloned().collect::<HashSet<_>>();
        let len = all_fields.len();
        // this syntax clones the fields
        vec![all_fields; len]
    };

    loop {
        let mut any_change = false;

        for field_index in 0..possible_fields.len() {
            // Check out the nth field of each ticket for each requirement
            for &ticket in valid_tickets.iter() {
                let field = *ticket.get(field_index).unwrap();
                // and check each requirement
                for (field_name, req) in input.requirements.iter() {
                    if !req.satisfied_by(field) {
                        // this field is invalid for this requirement!
                        // therefore this index/field guess is wrong
                        // println!(
                        //     "due to {}. removing {} from field_idx {}",
                        //     field, &field_name, field_index
                        // );
                        let removed = possible_fields
                            .get_mut(field_index)
                            .unwrap()
                            .remove(field_name);
                        if removed {
                            any_change = true;
                        }
                    }
                    // on to checking the next requirement for this field...
                }
            }
        }
        // OK we've checked every field. Let's check if we've identified anything
        // this holds identified fields and the index they came from
        let mut identified = Vec::new();
        for (idx, possibilities) in possible_fields.iter_mut().enumerate() {
            if possibilities.len() == 1 {
                // poggers~ we have zeroed in on something
                // or maybe, one-d in on something
                identified.push((possibilities.iter().next().unwrap().clone(), idx));
            }
        }
        // and remove the newly identified fields from everything
        if !identified.is_empty() {
            // println!("zeroed in on {:?}", &identified);
            for (idx, possibilities) in possible_fields.iter_mut().enumerate() {
                for (ided_field, ided_idx) in identified.iter() {
                    // don't knock out the identified field from the thing we identified it from
                    if idx != *ided_idx {
                        let removed = possibilities.remove(ided_field);
                        if removed {
                            any_change = true;
                        }
                    }
                }
            }
        }

        if !any_change {
            // we're done changing things
            break;
        }
    }

    // the actual mapping of indices to fields
    let mapping = possible_fields
        .iter()
        .enumerate()
        .map(|(idx, remainder)| {
            if remainder.len() == 1 {
                // poggers
                Ok(remainder.iter().next().unwrap().clone())
            } else {
                // oh no
                bail!("index #{}: leftover of {:?}", idx, remainder)
            }
        })
        .collect::<Result<Vec<_>>>()?;
    // invert it
    let mapping = mapping
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            println!("- #{} -> {}", idx + 1, field);
            (field, idx)
        })
        .collect::<HashMap<_, _>>();

    let answer: u64 = mapping
        .iter()
        .filter_map(|(field_name, field_idx)| {
            if !field_name.starts_with("departure") {
                // we don't care
                return None;
            }
            // get my field
            let myfield = *input.my_ticket.get(*field_idx).unwrap();
            Some(myfield as u64)
        })
        .product();
    println!("answer: {}", answer);

    Ok(())
}
 