use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone)]
enum Operation {
    Accumulator,
    Jump,
    NoOp,
}

impl From<&str> for Operation {
    fn from(operation_str: &str) -> Self {
        match operation_str {
            "acc" => Self::Accumulator,
            "jmp" => Self::Jump,
            "nop" => Self::NoOp,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl From<&str> for Instruction {
    fn from(instruction_str: &str) -> Self {
        let mut iter = instruction_str.split_whitespace();

        let operation: Operation = iter.next().unwrap().into();

        let argument = iter.next().unwrap().parse::<isize>().unwrap();

        Self {
            operation,
            argument,
        }
    }
}

struct Computer(Vec<Instruction>);

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Computer {
    Computer(input.lines().map(|line| line.into()).collect())
}

impl Computer {
    fn execute(&self) -> Result<isize, isize> {
        let mut visited = HashSet::new();

        let mut accumulator = 0;
        let mut i = 0;

        while i < self.0.len() {
            if visited.contains(&i) {
                return Err(accumulator);
            }

            visited.insert(i);

            match self.0[i].operation {
                Operation::Accumulator => {
                    accumulator += self.0[i].argument;
                    i += 1;
                }
                Operation::Jump => i = (i as isize + self.0[i].argument) as usize,
                Operation::NoOp => i += 1,
            }
        }

        Ok(accumulator)
    }
}

fn flip_operation(operation: &Operation) -> Operation {
    match operation {
        Operation::Jump => Operation::NoOp,
        Operation::NoOp => Operation::Jump,
        _ => operation.clone(),
    }
}

#[aoc(day8, part1)]
fn part1(computer: &Computer) -> isize {
    computer.execute().unwrap_err()
}

#[aoc(day8, part2)]
fn part2(computer: &Computer) -> isize {
    for i in 0..computer.0.len() {
        let instructions = &computer.0;

        match instructions[i].operation {
            Operation::Jump | Operation::NoOp => {
                let mut fixed_instructions = instructions.clone();
                fixed_instructions[i].operation = flip_operation(&fixed_instructions[i].operation);
                let fixed_computer = Computer(fixed_instructions);

                if let Ok(result) = fixed_computer.execute() {
                    return result;
                }
            }
            _ => continue,
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 8);
    }
}
