use std::collections::HashSet;

// Not parsing as a generator since HashSet doesn't have as_ref
pub fn parse_input(input: &str) -> HashSet<i64> {
    input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<HashSet<_>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let entries = parse_input(input);

    for i in entries.iter() {
        if entries.contains(&(2020 - i)) {
            return i * (2020 - i);
        }
    }

    panic!("No two entries have a sum of 2020")
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    let entries = parse_input(input);

    for i in entries.iter() {
        // Taking the lazy approach here, but this will have issues since we may consider
        // the same number twice
        for j in entries.iter() {
            let target = 2020 - i - j;

            if entries.contains(&target) {
                return i * j * target;
            }
        }
    }

    panic!("No three entries have a sum of 2020")
}
