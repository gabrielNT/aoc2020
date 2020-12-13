#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (usize, Vec<i64>) {
    let min_timestamp = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
    let mut bus_lines = Vec::new();

    for l in input.lines().nth(1).unwrap().split(",") {
        match l {
            "x" => bus_lines.push(0),
            _ => bus_lines.push(l.parse::<i64>().unwrap()),
        }
    }

    (min_timestamp, bus_lines)
}

#[aoc(day13, part1)]
pub fn part1(input: &(usize, Vec<i64>)) -> i64 {
    let mut bus_multiples = Vec::new();
    for b in input.1.iter() {
        if b < &1 {
            continue;
        }

        let mut i = 2;
        loop {
            let b_mult = b * i;
            if b_mult >= input.0 as i64 {
                bus_multiples.push((b_mult, b.to_owned()));
            }
            if b_mult >= 2 * input.0 as i64 {
                break;
            }

            i += 1;
        }
    }

    bus_multiples.sort();
    for b in bus_multiples.iter() {
        if b.0 >= input.0 as i64 {
            return b.1 * (b.0 - input.0 as i64);
        }
    }

    panic!("No answer found for part1");
}

#[aoc(day13, part2)]
pub fn part2(input: &(usize, Vec<i64>)) -> i64 {
    let mut divisors = Vec::new();
    let mut remainders = Vec::new();

    for i in 0..input.1.len() {
        if input.1[i] != 0 {
            divisors.push(input.1[i]);
            remainders.push(input.1[i] - i as i64)
        }
    }

    crt(divisors, remainders)
}

// Helpers
fn crt(divisors: Vec<i64>, remainders: Vec<i64>) -> i64 {
    let mut product: i64 = 1;
    for i in 0..divisors.len() {
        product *= divisors[i];
    }

    let mut total = 0;
    for i in 0..divisors.len() {
        let partial_product = product / divisors[i];
        let inverse = compute_inverse(partial_product, divisors[i]).unwrap();
        total += remainders[i] * partial_product * inverse;
    }

    total % product
}

pub fn compute_inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator("939\n7,13,x,x,59"),
            (939 as usize, vec![7, 13, 0, 0, 59])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("939\n7,13,x,x,59,x,31,19")), 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator("939\n7,13,x,x,59,x,31,19")), 1068781);
    }
}
