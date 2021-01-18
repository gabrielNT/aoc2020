#[derive(Debug, PartialEq, Copy, Clone)]
enum OpType {
    Add,
    Mul,
    LeftParen,
    RightParen,
    Operand,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Op {
    op_type: OpType,
    value: Option<usize>,
    precedence: usize,
}

fn shunting_yard(expr: &str, add_precedence: usize, mul_precedence: usize) -> Vec<Op> {
    let mut output: Vec<Op> = Vec::new();
    let mut operators: Vec<Op> = Vec::new();

    // Parses the string and constructs the result at the same time
    let subs_string = expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("  ", " ");
    let mut number = "".to_string();
    let mut was_digit = false;

    for (i, c) in subs_string.chars().enumerate() {
        // Parse digits. This was done in place of peeking the next element
        if c.is_digit(10) {
            number.insert(0, c);
            was_digit = true;
        }
        if was_digit && (!c.is_digit(10) || i == subs_string.len() - 1) {
            output.push(Op {
                op_type: OpType::Operand,
                value: Some(number.chars().rev().collect::<String>().parse().unwrap()),
                precedence: 0,
            });
            was_digit = false;
            number = "".to_string();
        }

        // Handle operators
        let op;
        match c {
            '+' => {
                op = Op {
                    op_type: OpType::Add,
                    value: None,
                    precedence: add_precedence,
                }
            }
            '*' => {
                op = Op {
                    op_type: OpType::Mul,
                    value: None,
                    precedence: mul_precedence,
                }
            }
            '(' => {
                op = Op {
                    op_type: OpType::LeftParen,
                    value: None,
                    precedence: 100,
                }
            }
            ')' => {
                op = Op {
                    op_type: OpType::RightParen,
                    value: None,
                    precedence: 100,
                }
            }
            _ => continue,
        }

        if op.op_type == OpType::Add || op.op_type == OpType::Mul {
            if !operators.is_empty() && operators[operators.len() - 1].precedence <= op.precedence {
                output.push(operators.pop().unwrap());
            }
            operators.push(op);
        } else if op.op_type == OpType::LeftParen {
            operators.push(op);
        } else if op.op_type == OpType::RightParen {
            let mut pop_op = operators.pop().unwrap();
            while pop_op.op_type != OpType::LeftParen {
                output.push(pop_op);
                pop_op = operators.pop().unwrap();
            }
        }
    }

    // Pop remaining operators
    while !operators.is_empty() {
        output.push(operators.pop().unwrap());
    }

    output
}

fn eval(postfix_expr: Vec<Op>) -> usize {
    let mut op_stack = Vec::new();
    for i in postfix_expr.iter() {
        match i.op_type {
            OpType::Operand => op_stack.push(i.value.unwrap()),
            OpType::Add => {
                let op_result = op_stack.pop().unwrap() + op_stack.pop().unwrap();
                op_stack.push(op_result);
            }
            OpType::Mul => {
                let op_result = op_stack.pop().unwrap() * op_stack.pop().unwrap();
                op_stack.push(op_result);
            }
            _ => panic!("Invalid expression!"),
        }
    }

    op_stack.pop().unwrap()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> usize {
    let mut sum = 0;

    for l in input.lines() {
        sum += eval(shunting_yard(l, 1, 1));
    }

    sum
}

#[aoc(day18, part2)]
fn part2(input: &str) -> usize {
    let mut sum = 0;

    for l in input.lines() {
        sum += eval(shunting_yard(l, 1, 2));
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }
}
