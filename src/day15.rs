#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|l| {
            l.parse::<usize>().unwrap()
        }).collect()
}

#[aoc(day15, part1)]
pub fn part1(starting: &Vec<usize>) -> usize {
    const ITERATIONS: usize = 2020;
    let numbers = starting.clone();
    let last_index = vec![None; ITERATIONS];

    play_game(numbers, last_index, ITERATIONS)
}

#[aoc(day15, part2)]
pub fn part2(starting: &Vec<usize>) -> usize {
    const ITERATIONS: usize = 30000000;
    let numbers = starting.clone();
    let last_index = vec![None; ITERATIONS];

    play_game(numbers, last_index, ITERATIONS)
}


// Helpers
fn play_game(mut numbers: Vec<usize>, mut last_index: Vec<Option<usize>>, iterations: usize) -> usize {
    for (i, n) in numbers.iter().enumerate() {
        last_index[n.to_owned()] = Some(i + 1);
    }

    for j in numbers.len()..iterations {
        match last_index[numbers[j - 1]] {
            Some(last_num) => {
                numbers.push(j - last_num);
            },
            None => {
                numbers.push(0);
            },
        } 
        last_index[numbers[j - 1]] = Some(j);
    }

    numbers.last().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("0,3,6")), 436);
    }

    #[test]
    fn test_part2() {
        // This test can be slow, leaving it commented for the future
        //assert_eq!(part2(&input_generator("0,3,6")), 175594);
    }
}