use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use std::result::Result as StdResult;

pub struct Executor {
    acc: i64,

    /// Program counter: the index of the instruction we're about to execute
    pc: usize,
    /// The instructions
    program: Vec<Instruction>,
}

impl Executor {
    /// Create a new executor from the input
    pub fn new(i: &str) -> Result<Self> {
        let program = Executor::parse_program(i)?;
        Ok(Executor::new_from_program(program))
    }

    /// Create a new executor from a pre-parsed input
    pub fn new_from_program(program: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            pc: 0,
            program,
        }
    }

    /// Parse an input into a program.
    pub fn parse_program(i: &str) -> Result<Vec<Instruction>> {
        i.lines()
            .enumerate()
            .map(|(idx, line)| line.parse().context(format!("on line {}", idx + 1)))
            .collect::<Result<_>>()
    }

    /// Get the value of the accumulator.
    pub fn get_accumulator(&self) -> i64 {
        self.acc
    }

    /// Execute one opcode.
    pub fn step(&mut self) {
        let Instruction {
            operation,
            argument,
        } = &self.program[self.pc];
        self.pc += 1;
        match operation {
            Operation::Acc => self.acc += argument,
            // Subtract 1 because we already moved PC
            Operation::Jmp => self.pc = ((self.pc as i64) + argument) as usize - 1,
            Operation::Nop => {}
        }
    }

    /// Create an Executor from the input internally and run until just before one index would be executed twice.
    /// Returns the value of the accumulator at that point (or an error).
    pub fn run_until_loop(i: &str) -> Result<i64> {
        let mut exe = Executor::new(i)?;
        let mut indexes_execed = HashSet::new();

        while !indexes_execed.contains(&exe.pc) {
            indexes_execed.insert(exe.pc);
            exe.step();
        }

        Ok(exe.acc)
    }

    /// Run until either the end,
    /// or just before one index would be executed twice.
    /// Returns the value of the accumulator at that point.
    /// Ok if it terminated normally, Err if it terminated because it was going to loop.
    pub fn run_until_end_or_loop(&mut self) -> StdResult<i64, i64> {
        let mut indexes_execed = HashSet::new();

        loop {
            if indexes_execed.contains(&self.pc) {
                return Err(self.acc);
            }
            if self.pc >= self.program.len() {
                return Ok(self.acc);
            }

            indexes_execed.insert(self.pc);
            self.step();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub argument: i64,
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_ascii_whitespace().collect::<Vec<_>>();
        match split.as_slice() {
            [op, arg] => {
                let operation = op.parse()?;
                let argument = arg.parse()?;
                Ok(Instruction {
                    operation,
                    argument,
                })
            }
            oh_no => bail!("expected 2 words, found {}", oh_no.len()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    /// Add/sub to the accumulator
    Acc,
    /// Jump by this offset
    Jmp,
    /// Do nothing
    Nop,
}

impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            oh_no => bail!("unknown opcode `{}`", oh_no),
        })
    }
}
