#[derive(Debug, PartialEq)]
pub struct Pass {
    ver_code: String,
    hor_code: String,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Pass> {
    input
        .lines()
        .map(|l| Pass {
            ver_code: l[..7].to_string(),
            hor_code: l[7..].to_string(),
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Pass>) -> usize {
    let mut max: usize = 0;

    for p in input.iter() {
        let curr = get_seat_id(partition(&p.ver_code, 0, 127), partition(&p.hor_code, 0, 7));
        if curr > max {
            max = curr;
        }
    }

    max
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Pass>) -> usize {
    let mut seats = [false; 1024];

    for p in input.iter() {
        seats[get_seat_id(partition(&p.ver_code, 0, 127), partition(&p.hor_code, 0, 7))] = true;
    }

    for (id, filled) in seats.iter().enumerate() {
        if id < 8 || id > 1015 {
            continue;
        }

        if filled == &false && seats[id - 1] == true && seats[id + 1] == true {
            return id;
        }
    }

    panic!("No valid seat found!");
}

// Helpers
fn partition(code: &String, mut min: u8, mut max: u8) -> u8 {
    // Not really doing any sanity checks here, it might fail with invalid input
    for p in code.chars() {
        let mid: u8 = min + (max - min) / 2;

        match p {
            'F' | 'L' => max = mid,
            'B' | 'R' => min = mid,
            _ => panic!("Invalid code!"),
        }
    }

    max
}

fn get_seat_id(row: u8, column: u8) -> usize {
    usize::from(row) * 8 + usize::from(column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator("BFFFBBFRRR\nFFFBBBFRRR"),
            vec![
                Pass {
                    ver_code: "BFFFBBF".to_string(),
                    hor_code: "RRR".to_string()
                },
                Pass {
                    ver_code: "FFFBBBF".to_string(),
                    hor_code: "RRR".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_partition() {
        // Test vertical partition
        assert_eq!(partition(&"BFFFBBF".to_string(), 0, 127), 70);
        assert_eq!(partition(&"FFFBBBF".to_string(), 0, 127), 14);
        assert_eq!(partition(&"BBFFBBF".to_string(), 0, 127), 102);
        assert_eq!(partition(&"FBFBBFF".to_string(), 0, 127), 44);

        // Test horizontal partition
        assert_eq!(partition(&"RRR".to_string(), 0, 7), 7);
        assert_eq!(partition(&"RLL".to_string(), 0, 7), 4);
        assert_eq!(partition(&"RLR".to_string(), 0, 7), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator(&"BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL")),
            820
        );
    }
}
