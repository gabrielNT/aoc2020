pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut adapters = input_generator(input);
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    let mut one_diffs: usize = 0;
    let mut three_diffs: usize = 0;

    for i in 1..adapters.len() {
        let diff = adapters[i] - adapters[i - 1];
        match diff {
            0 | 2 => (),
            1 => one_diffs += 1,
            3 => three_diffs += 1,
            _ => panic!("Invalid input!"),
        }
    }

    one_diffs * three_diffs
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut adapters = input_generator(input);
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    let mut combinations = vec![1; adapters.len()];
    for i in (0..adapters.len() - 3).rev() {
        if adapters[i] == 10 {
            println!("s");
        }

        if adapters[i + 3] - adapters[i] <= 3 {
            combinations[i] = combinations[i + 3] + combinations[i + 2] + combinations[i + 1];
        } else if adapters[i + 2] - adapters[i] <= 3 {
            combinations[i] = combinations[i + 2] + combinations[i + 1];
        } else if adapters[i + 1] - adapters[i] <= 3 {
            combinations[i] = combinations[i + 1];
        }
    }

    combinations[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3"), 
            220
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4"), 8);
        assert_eq!(
            part2("28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3"), 
            19208
        );
    }
}
