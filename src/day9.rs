#[derive(Debug, PartialEq)]
pub struct Xmas {
    numbers: Vec<usize>,
    first_valid_idx: usize,
}

impl Xmas {
    pub fn check_next(&mut self) -> Option<(usize, usize)> {
        // For 25 inputs brute-forcing should be fine
        for i in self.first_valid_idx - 25..self.first_valid_idx - 1 {
            for j in i + 1..self.first_valid_idx {
                if self.numbers[i] + self.numbers[j] == self.numbers[self.first_valid_idx] {
                    self.first_valid_idx += 1;
                    return Some((i, j));
                }
            }
        }

        self.first_valid_idx += 1;
        None
    }
}

pub fn input_generator(input: &str) -> Xmas {
    let mut numbers = Vec::new();
    for l in input.lines() {
        numbers.push(l.parse::<usize>().unwrap());
    }

    Xmas {
        numbers,
        first_valid_idx: 25,
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut code = input_generator(input);
    while code.first_valid_idx < code.numbers.len() {
        match code.check_next() {
            Some(_) => (),
            None => return code.numbers[code.first_valid_idx - 1],
        }
    }

    panic!("All inputs were valid!");
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let code = input_generator(input);
    let mut i: usize = 0;
    let mut curr: usize = code.numbers[i];
    let target: usize = 675280050;

    for j in 1..code.numbers.len() {
        while curr > target && i < j - 1 {
            curr -= code.numbers[i];
            i += 1;
        }

        if curr == target {
            let mut max = 0;
            let mut min = 4294967296;

            for k in i..j {
                if code.numbers[k] > max {
                    max = code.numbers[k];
                }
                if code.numbers[k] < min {
                    min = code.numbers[k];
                }
            }

            return min + max;
        }

        if j < code.numbers.len() {
            curr += code.numbers[j];
        }
    }

    panic!("No sequence sums to the target!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator("35\n20\n15\n25"),
            Xmas {
                numbers: vec![35, 20, 15, 25],
                first_valid_idx: 25,
            }
        );
    }
}
