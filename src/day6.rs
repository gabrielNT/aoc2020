#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(usize, [usize; 26])> {
    input
        .split("\n\n")
        .map(|p| {
            let mut total = 1;
            let mut answers = [0; 26];
            for c in p.chars() {
                match c {
                   'a'..='z' => answers[c as usize - 97] += 1,
                   '\n' => total += 1,
                    _ => panic!("Invalid input!"),
                } 
            }
            (total, answers)
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<(usize, [usize; 26])>) -> usize {
    let mut total = 0;
    for (_, v) in input.iter() {
        total += count_trues(v, &1);
    }
    total
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<(usize, [usize; 26])>) -> usize {
    let mut total = 0;
    for (t, v) in input.iter() {
        total += count_trues(v, &t);
    }
    total
}

// Helpers
fn count_trues(array: &[usize; 26], count_needed: &usize) -> usize {
    let mut count = 0;
    for a in array.iter() {
        if a >= count_needed {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut answers: [usize; 26] = [0; 26];
        answers[0] = 1;
        answers[1] = 1;
        answers[2] = 1;

        assert_eq!(input_generator("abc\n\na\nb\nc"), vec!((1, answers), (3, answers)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb")), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb")), 6);
    }
}