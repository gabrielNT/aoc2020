use regex::Regex;

pub struct Password {
    key: String,
    policy_char: char,
    policy_first: usize,
    policy_last: usize,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let re = Regex::new(r"(\d+)-(\d+) ([A-Za-z]): ([A-Za-z]+)").unwrap();
            let cap = re.captures(l).unwrap();
            Password {
                key: cap[4].to_string(),
                policy_char: cap[3].chars().next().expect("String is empty"),
                policy_first: cap[1].parse::<usize>().unwrap(),
                policy_last: cap[2].parse::<usize>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Password>) -> usize {
    let mut valid_count = 0;
    for p in input {
        let char_count = count_chars(&p.key, &p.policy_char);
        if char_count >= p.policy_first && char_count <= p.policy_last {
            valid_count += 1;
        }
    }

    valid_count
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Password>) -> usize {
    let mut valid_count = 0;
    for p in input {
        // For this exercise considering only ASCII should be ok
        if (p.key.as_bytes()[p.policy_first - 1] as char == p.policy_char)
            ^ (p.key.as_bytes()[p.policy_last - 1] as char == p.policy_char)
        {
            valid_count += 1;
        }
    }

    valid_count
}

// Helpers
fn count_chars(input: &String, character: &char) -> usize {
    let mut count = 0;
    for c in input.chars() {
        if character == &c {
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
        let result = input_generator("1-3 a: abcde");
        assert_eq!(result[0].key, "abcde".to_string());
        assert_eq!(result[0].policy_char, 'a');
        assert_eq!(result[0].policy_first, 1);
        assert_eq!(result[0].policy_last, 3);
    }

    #[test]
    fn test_input_multiple() {
        let result = input_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_char_count() {
        assert_eq!(count_chars(&"aab".to_string(), &'a'), 2);
    }

    #[test]
    fn test_part1() {
        let result = part1(&input_generator(
            "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc",
        ));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = part2(&input_generator(
            "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc",
        ));
        assert_eq!(result, 1);
    }
}
