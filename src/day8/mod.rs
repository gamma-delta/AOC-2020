use anyhow::Result;

use crate::intcodent::{Executor, Operation};

const INPUT: &str = include_str!("input.txt");

#[test]
fn part1() -> Result<()> {
    let res = Executor::run_until_loop(INPUT)?;
    println!("result: {}", res);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let program = Executor::parse_program(INPUT)?;

    // For each index in the program, change its opcode.
    // and try to execute it
    for change_idx in 0..program.len() {
        let mut program = program.clone();
        let swapped_opcode = match &program[change_idx].operation {
            Operation::Jmp => Operation::Nop,
            Operation::Nop => Operation::Jmp,
            _ => continue,
        };
        program[change_idx].operation = swapped_opcode;

        let mut exe = Executor::new_from_program(program);
        let output = exe.run_until_end_or_loop();
        if let Ok(output) = output {
            println!(
                "by changing line {} to {:?}, output is {}",
                change_idx + 1,
                swapped_opcode,
                output
            );
            break;
        }
    }

    Ok(())
}

#[test]
fn part1_test() -> Result<()> {
    let input = r"nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";
    let res = Executor::run_until_loop(input)?;
    println!("result: {}", res);

    Ok(())
}
