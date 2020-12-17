use std::collections::HashMap;

use anyhow::{bail, Result};

use super::requirements::Requirement;

/// Representation of the Input
#[derive(Debug)]
pub struct Input {
    /// The requirements, mapping the key name to the Requirement
    pub requirements: HashMap<String, Requirement>,
    /// My ticket
    pub my_ticket: Vec<u32>,
    /// Other people's tickets
    pub other_tickets: Vec<Vec<u32>>,
}

impl Input {
    /// Parse the input string into an Input
    pub fn new(i: &str) -> Result<Self> {
        // Split this into sections
        let sections = i.split("\n\n").collect::<Vec<_>>();
        let (reqs, my_ticket, their_tickets) = match sections.as_slice() {
            [reqs, my_ticket, their_tickets] => (reqs, my_ticket, their_tickets),
            _ => bail!("could not split into sections"),
        };

        let reqs = parse_requirements(reqs)?;

        let my_tickets = parse_tickets(my_ticket)?;
        if my_tickets.len() != 1 {
            bail!("wrong number of my tickets, got {}", my_tickets.len())
        }
        // not sure why i have to do this clone ... can't i just consume the whole my_tickets?
        let my_ticket = my_tickets[0].clone();

        let other_tickets = parse_tickets(their_tickets)?;

        Ok(Self {
            requirements: reqs,
            my_ticket,
            other_tickets,
        })
    }
}

fn parse_requirements(i: &str) -> Result<HashMap<String, Requirement>> {
    i.lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<_>>();
            let (key, reqs) = match split.as_slice() {
                [key, reqs] => (key, reqs),
                _ => bail!("could not split requirement"),
            };
            let reqs_split = reqs.split_ascii_whitespace().collect::<Vec<_>>();
            let (req, reqs_rem) = Requirement::new(&reqs_split)?;
            if !reqs_rem.is_empty() {
                bail!(
                    "there was leftover after parsing a requirement: {:?}",
                    reqs_rem
                );
            }
            Ok(((*key).to_owned(), req))
        })
        .collect()
}

fn parse_tickets(i: &str) -> Result<Vec<Vec<u32>>> {
    // skip line #1 where it has the header
    // like `your ticket`
    i.lines()
        .skip(1)
        .map(|line| line.split(',').map(|num| Ok(num.parse()?)).collect())
        .collect()
}
