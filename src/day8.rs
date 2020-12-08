#[derive(Debug, PartialEq)]
pub enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    op_code: OpCode,
    arg: i32,
    count_executed: usize,
}

#[derive(Debug, PartialEq)]
pub struct HandheldConsole {
    program: Vec<Operation>,
    curr_op: usize,
    accumulator: i32,
}

impl HandheldConsole {
    pub fn new_with_program(program: Vec<Operation>) -> Self {
        HandheldConsole {
            program,
            curr_op: 0,
            accumulator: 0,
        }
    }

    pub fn clear(&mut self) {
        self.curr_op = 0;
        self.accumulator = 0;
        for op in self.program.iter_mut() {
            op.count_executed = 0;
        }
    }

    pub fn step(&mut self) {
        self.program[self.curr_op].count_executed += 1;

        let op = &self.program[self.curr_op];
        match op.op_code {
            OpCode::Acc => {
                self.accumulator += op.arg;
                self.curr_op += 1;
            }
            OpCode::Jmp => {
                self.curr_op = (self.curr_op as i32 + op.arg) as usize;
            }
            OpCode::Nop => {
                self.curr_op += 1;
            }
        }
    }

    pub fn run_and_break_on_repeat(&mut self) -> i32 {
        let mut previous_acc = self.accumulator;

        loop {
            if self.curr_op >= self.program.len() {
                break;
            }

            let repeat = self.program[self.curr_op].count_executed + 1;
            if repeat > 1 {
                break;
            }

            self.step();
            previous_acc = self.accumulator;
        }

        previous_acc
    }
}

pub fn input_generator(input: &str) -> HandheldConsole {
    let program = input
        .lines()
        .map(|l| match &l[..3] {
            "acc" => Operation {
                op_code: OpCode::Acc,
                arg: l[4..].trim().parse().unwrap(),
                count_executed: 0,
            },
            "jmp" => Operation {
                op_code: OpCode::Jmp,
                arg: l[4..].trim().parse().unwrap(),
                count_executed: 0,
            },
            "nop" => Operation {
                op_code: OpCode::Nop,
                arg: l[4..].trim().parse().unwrap(),
                count_executed: 0,
            },
            _ => panic!("Invalid input: {}", &l[..4]),
        })
        .collect();

    HandheldConsole::new_with_program(program)
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
    let mut console = input_generator(input);
    console.run_and_break_on_repeat()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
    let mut console = input_generator(input);
    for i in 0..console.program.len() {
        console.clear();

        match console.program[i].op_code {
            OpCode::Acc => continue,
            OpCode::Jmp => console.program[i].op_code = OpCode::Nop,
            OpCode::Nop => console.program[i].op_code = OpCode::Jmp,
        }

        console.run_and_break_on_repeat();
        println!("{} : {}", i, console.curr_op);
        if console.curr_op >= console.program.len() {
            return console.accumulator;
        }

        match console.program[i].op_code {
            OpCode::Acc => continue,
            OpCode::Jmp => console.program[i].op_code = OpCode::Nop,
            OpCode::Nop => console.program[i].op_code = OpCode::Jmp,
        }
    }

    panic!("Couldn't fix the loop!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator("nop +0\nacc -1"),
            HandheldConsole {
                program: vec![
                    Operation {
                        op_code: OpCode::Nop,
                        arg: 0,
                        count_executed: 0,
                    },
                    Operation {
                        op_code: OpCode::Acc,
                        arg: -1,
                        count_executed: 0,
                    }
                ],
                curr_op: 0,
                accumulator: 0,
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"),
            5
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"),
            8
        );
    }
}
