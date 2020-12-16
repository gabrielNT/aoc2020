use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Rule {
    field_name: String,
    fst_valid_first_idx: usize,
    fst_valid_last_idx: usize,
    snd_valid_first_idx: usize,
    snd_valid_last_idx: usize,
}

#[derive(Debug, PartialEq)]
pub struct Ticket {
    fields: Vec<usize>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> (Vec<Ticket>, Vec<Rule>) {
    let rules_regex = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut rules: Vec<Rule> = Vec::new();
    let mut tickets = Vec::new();
    let mut tickets_stage = false;

    for l in input.lines() {
        if l == "your ticket:" || l == "nearby tickets:" || l == "" {
            tickets_stage = true;
            continue;
        }

        if tickets_stage {
            tickets.push(Ticket {
                fields: l.split(",").map(|c| c.parse::<usize>().unwrap()).collect(),
            });
        } else {
            let captures = rules_regex.captures(l).unwrap();
            rules.push(Rule {
                field_name: captures[1].to_string(),
                fst_valid_first_idx: captures[2].parse::<usize>().unwrap(),
                fst_valid_last_idx: captures[3].parse::<usize>().unwrap(),
                snd_valid_first_idx: captures[4].parse::<usize>().unwrap(),
                snd_valid_last_idx: captures[5].parse::<usize>().unwrap(),
            });
        }
    }

    (tickets, rules)
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Ticket>, Vec<Rule>)) -> usize {
    remove_invalid(&input.0, &input.1).0
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Ticket>, Vec<Rule>)) -> usize {
    let valid_idxs = remove_invalid(&input.0, &input.1).1;
    let mut possible: Vec<Vec<bool>> = vec![vec![true; input.1.len()]; input.0[0].fields.len()];

    for i in 1..valid_idxs.len() {
        let ticket = &input.0[valid_idxs[i]];

        for (field_num, field) in ticket.fields.iter().enumerate() {
            for (rule_num, rule) in input.1.iter().enumerate() {
                if !((field >= &rule.fst_valid_first_idx && field <= &rule.fst_valid_last_idx)
                    || (field >= &rule.snd_valid_first_idx && field <= &rule.snd_valid_last_idx))
                {
                    // We could terminate this early
                    possible[field_num][rule_num] = false;
                }
            }
        }
    }

    loop {
        let mut match_count = 0;

        for i in 0..possible.len() {
            let (is_single, idx) = single_true(&possible[i]);
            if is_single {
                match_count += 1;
                for (curr_idx, p) in possible.iter_mut().enumerate() {
                    // Do no clear the field itself
                    if curr_idx == i {
                        continue;
                    }
                    p[idx] = false;
                }
            }
        }

        if match_count >= possible.len() {
            break;
        }
    }

    // After this point there needs to be only one true per field in possible
    let mut result = 1;
    for (i, r) in input.1.iter().enumerate() {
        if r.field_name.contains("departure") {
            for (dep_idx, f) in possible.iter().enumerate() {
                if f[i] == true {
                    result *= input.0[0].fields[dep_idx];
                }
            }
        }
    }

    result
}

// Helpers
fn remove_invalid(tickets: &Vec<Ticket>, rules: &Vec<Rule>) -> (usize, Vec<usize>) {
    let mut error_count = 0;
    let mut idx_vec = Vec::new();

    for (idx, ticket) in tickets.iter().enumerate() {
        let mut is_ticket_valid = true;

        for field in ticket.fields.iter() {
            let mut is_field_valid = false;
            for rule in rules.iter() {
                if (field >= &rule.fst_valid_first_idx && field <= &rule.fst_valid_last_idx)
                    || (field >= &rule.snd_valid_first_idx && field <= &rule.snd_valid_last_idx)
                {
                    is_field_valid = true;
                    break;
                }
            }

            if !is_field_valid {
                error_count += field;
                is_ticket_valid = false;
            }
        }

        if is_ticket_valid {
            idx_vec.push(idx);
        }
    }

    (error_count, idx_vec)
}

fn single_true(input: &Vec<bool>) -> (bool, usize) {
    let mut true_count = 0;
    let mut true_idx = 0;
    for (i, b) in input.iter().enumerate() {
        if b == &true {
            true_count += 1;
            true_idx = i;

            if true_count > 1 {
                return (false, 0);
            }
        }
    }

    (true, true_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator(
                "class: 1-3 or 5-7\n\
                              row: 6-11 or 33-44\n\
                              seat: 13-40 or 45-50\n\n\
                              your ticket:\n\
                              7,1,14\n\n\
                              nearby tickets:\n\
                              7,3,47\n\
                              40,4,50\n\
                              55,2,20\n\
                              38,6,12"
            ),
            (
                vec![
                    Ticket {
                        fields: vec![7, 1, 14],
                    },
                    Ticket {
                        fields: vec![7, 3, 47],
                    },
                    Ticket {
                        fields: vec![40, 4, 50],
                    },
                    Ticket {
                        fields: vec![55, 2, 20],
                    },
                    Ticket {
                        fields: vec![38, 6, 12],
                    }
                ],
                vec![
                    Rule {
                        field_name: "class".to_string(),
                        fst_valid_first_idx: 1,
                        fst_valid_last_idx: 3,
                        snd_valid_first_idx: 5,
                        snd_valid_last_idx: 7,
                    },
                    Rule {
                        field_name: "row".to_string(),
                        fst_valid_first_idx: 6,
                        fst_valid_last_idx: 11,
                        snd_valid_first_idx: 33,
                        snd_valid_last_idx: 44,
                    },
                    Rule {
                        field_name: "seat".to_string(),
                        fst_valid_first_idx: 13,
                        fst_valid_last_idx: 40,
                        snd_valid_first_idx: 45,
                        snd_valid_last_idx: 50,
                    },
                ]
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&input_generator(
                "class: 1-3 or 5-7\n\
                              row: 6-11 or 33-44\n\
                              seat: 13-40 or 45-50\n\n\
                              your ticket:\n\
                              7,1,14\n\n\
                              nearby tickets:\n\
                              7,3,47\n\
                              40,4,50\n\
                              55,2,20\n\
                              38,6,12"
            )),
            71
        );
    }

    #[test]
    fn test_idx_vec() {
        let (tickets, rules) = input_generator(
            "class: 1-3 or 5-7\n\
                   row: 6-11 or 33-44\n\
                   seat: 13-40 or 45-50\n\n\
                   your ticket:\n\
                   7,1,14\n\n\
                   nearby tickets:\n\
                   7,3,47\n\
                   40,4,50\n\
                   55,2,20\n\
                   38,6,12",
        );

        assert_eq!(remove_invalid(&tickets, &rules).1, vec![0, 1]);
    }
}
