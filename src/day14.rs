use regex::Regex;
use std::collections::HashMap;

const MEMORY_SIZE: usize = 100000; // Is there an address bigger than this in the input?
const MAX_36_BITS: u64 = u64::max_value() >> (64 - 36);

#[derive(Debug, PartialEq)]
pub struct Instruction {
    value: u64,
    address: usize,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    ones_mask: u64,
    zeroes_mask: u64,
    xs_mask: u64,
    instructions: Vec<Instruction>,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Program> {
    let instruction_re = Regex::new(r"mem\[(.*)\] = (.*)").unwrap();
    let mut program_vec = Vec::new();

    for block in input.split("mask = ") {
        let mut ones_mask = 0;
        let mut zeroes_mask = 0;
        let mut instructions = Vec::new();

        for l in block.lines() {
            if !instruction_re.is_match(l) {
                for (i, c) in l.chars().enumerate() {
                    match c {
                        '0' => zeroes_mask |= 1 << (35 - i),
                        '1' => ones_mask |= 1 << (35 - i),
                        'X' => (),
                        _ => panic!("Invalid mask input"),
                    }
                }
            } else {
                let cap = instruction_re.captures(l).unwrap();
                instructions.push(Instruction {
                    value: cap[2].parse::<u64>().unwrap(),
                    address: cap[1].parse::<usize>().unwrap(),
                });
            }
        }

        if !instructions.is_empty() {
            program_vec.push(Program {
                ones_mask,
                zeroes_mask,
                xs_mask: ones_mask | zeroes_mask, // This is an inverted bit mask
                instructions,
            });
        }
    }

    program_vec
}

#[aoc(day14, part1)]
pub fn part1(programs: &Vec<Program>) -> u64 {
    let mut memory = [0 as u64; MEMORY_SIZE];

    for p in programs {
        for i in p.instructions.iter() {
            memory[i.address] = (i.value | p.ones_mask) & !p.zeroes_mask;
        }
    }

    check_sums(memory)
}

#[aoc(day14, part2)]
pub fn part2(programs: &Vec<Program>) -> u64 {
    // For part2 the full 32 bits are needed
    let mut memory = HashMap::new();

    for p in programs {
        for inst in p.instructions.iter() {
            let base_addr = (inst.address as u64 | p.ones_mask) & p.xs_mask;

            let mut curr_mask = p.xs_mask;
            loop {
                memory.insert(base_addr | (!curr_mask & MAX_36_BITS), inst.value);
                if curr_mask & MAX_36_BITS == MAX_36_BITS {
                    break;
                }
                curr_mask = (curr_mask + 1) | p.xs_mask;
            }
        }
    }

    check_sums_hash(memory)
}

// Helpers
fn check_sums(memory: [u64; MEMORY_SIZE]) -> u64 {
    let mut sum = 0;
    for i in memory.iter() {
        sum += i;
    }

    sum
}

fn check_sums_hash(memory: HashMap<u64, u64>) -> u64 {
    let mut sum = 0;
    for (_, value) in memory.iter() {
        sum += value;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                 mem[8] = 11\n\
                 mem[7] = 101\n\
                 mem[8] = 0\n\
                 mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                 mem[8] = 11"
            ),
            vec![
                Program {
                    ones_mask: 64,
                    zeroes_mask: 2,
                    xs_mask: 66,
                    instructions: vec![
                        Instruction {
                            value: 11,
                            address: 8
                        },
                        Instruction {
                            value: 101,
                            address: 7
                        },
                        Instruction {
                            value: 0,
                            address: 8
                        }
                    ]
                },
                Program {
                    ones_mask: 64,
                    zeroes_mask: 2,
                    xs_mask: 66,
                    instructions: vec![Instruction {
                        value: 11,
                        address: 8
                    }]
                }
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                 mem[8] = 11\n\
                 mem[7] = 101\n\
                 mem[8] = 0"
            )),
            165
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&input_generator(
                "mask = 000000000000000000000000000000X1001X\n\
                 mem[42] = 100\n\
                 mask = 00000000000000000000000000000000X0XX\n\
                 mem[26] = 1"
            )),
            208
        );
    }
}
