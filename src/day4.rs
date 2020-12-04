use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn new() -> Self {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|p| {
            let mut passport = Passport::new();
            for e in p.split_whitespace() {
                match &e[0..3] {
                    "byr" => passport.birth_year = Some(e[4..].to_string()),
                    "iyr" => passport.issue_year = Some(e[4..].to_string()),
                    "eyr" => passport.expiration_year = Some(e[4..].to_string()),
                    "hgt" => passport.height = Some(e[4..].to_string()),
                    "hcl" => passport.hair_color = Some(e[4..].to_string()),
                    "ecl" => passport.eye_color = Some(e[4..].to_string()),
                    "pid" => passport.passport_id = Some(e[4..].to_string()),
                    "cid" => passport.country_id = Some(e[4..].to_string()),
                    _ => panic!("Invalid input!"),
                }
            }
            passport
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<Passport>) -> usize {
    let mut valid_count = 0;

    for p in input.iter() {
        if !p.birth_year.is_some()
            || !p.issue_year.is_some()
            || !p.expiration_year.is_some()
            || !p.height.is_some()
            || !p.hair_color.is_some()
            || !p.eye_color.is_some()
            || !p.passport_id.is_some()
        {
            continue;
        }
        valid_count += 1;
    }
    valid_count
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<Passport>) -> usize {
    let mut valid_count = 0;

    for p in input.iter() {
        match &p.birth_year {
            Some(b) => {
                let b_int = b.parse::<usize>().unwrap();
                if b_int < 1920 || b_int > 2002 {
                    continue;
                }
            }
            None => continue,
        }

        match &p.issue_year {
            Some(b) => {
                let b_int = b.parse::<usize>().unwrap();
                if b_int < 2010 || b_int > 2020 {
                    continue;
                }
            }
            None => continue,
        }

        match &p.expiration_year {
            Some(b) => {
                let b_int = b.parse::<usize>().unwrap();
                if b_int < 2020 || b_int > 2030 {
                    continue;
                }
            }
            None => continue,
        }

        match &p.height {
            Some(h) => match &h[h.len() - 2..] {
                "cm" => {
                    let h_int = h[..h.len() - 2].parse::<usize>().unwrap();
                    if h_int < 150 || h_int > 193 {
                        continue;
                    }
                }
                "in" => {
                    let h_int = h[..h.len() - 2].parse::<usize>().unwrap();
                    if h_int < 59 || h_int > 76 {
                        continue;
                    }
                }
                _ => continue,
            },
            None => continue,
        }

        match &p.hair_color {
            Some(hc) => {
                let re = Regex::new(r"^#([0-9a-fA-F]{3}){1,2}$").unwrap();
                if !re.is_match(hc) {
                    continue;
                }
            }
            None => continue,
        }

        match &p.eye_color {
            Some(ec) => match &ec[..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                _ => continue,
            },
            None => continue,
        }

        match &p.passport_id {
            Some(pid) => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                if !re.is_match(pid) {
                    continue;
                }
            }
            None => continue,
        }

        valid_count += 1;
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                     byr:1937 iyr:2017 cid:147 hgt:183cm\n\
                     \n\
                     iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                     hcl:#cfa07d byr:1929";
        let mut expected: Vec<Passport> = Vec::new();
        expected.push(Passport {
            birth_year: Some("1937".to_string()),
            issue_year: Some("2017".to_string()),
            expiration_year: Some("2020".to_string()),
            height: Some("183cm".to_string()),
            hair_color: Some("#fffffd".to_string()),
            eye_color: Some("gry".to_string()),
            passport_id: Some("860033327".to_string()),
            country_id: Some("147".to_string()),
        });
        expected.push(Passport {
            birth_year: Some("1929".to_string()),
            issue_year: Some("2013".to_string()),
            expiration_year: Some("2023".to_string()),
            height: None,
            hair_color: Some("#cfa07d".to_string()),
            eye_color: Some("amb".to_string()),
            passport_id: Some("028048884".to_string()),
            country_id: Some("350".to_string()),
        });

        assert_eq!(expected, input_generator(input));
    }

    #[test]
    fn test_part1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                     byr:1937 iyr:2017 cid:147 hgt:183cm\n\
                     \n\
                     iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                     hcl:#cfa07d byr:1929\n\
                     \n\
                     hcl:#ae17e1 iyr:2013\n\
                     eyr:2024\n\
                     ecl:brn pid:760753108 byr:1931\n\
                     hgt:179cm\n\
                     \n\
                     hcl:#cfa07d eyr:2025 pid:166559648\n\
                     iyr:2011 ecl:brn hgt:59in";

        assert_eq!(part1(&input_generator(input)), 2);
    }

    #[test]
    fn test_part2() {
        let input = "eyr:1972 cid:100\n\
                     hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
                     \n\
                     iyr:2019\n\
                     hcl:#602927 eyr:1967 hgt:170cm\n\
                     ecl:grn pid:012533040 byr:1946\n\
                     \n\
                     hcl:dab227 iyr:2012\n\
                     ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
                     \n\
                     hgt:59cm ecl:zzz\n\
                     eyr:2038 hcl:74454a iyr:2023\n\
                     pid:3556412378 byr:2007\n\
                     pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                     hcl:#623a2f\n\
                     \n\
                     eyr:2029 ecl:blu cid:129 byr:1989\n\
                     iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
                     \n\
                     hcl:#888785\n\
                     hgt:164cm byr:2001 iyr:2015 cid:88\n\
                     pid:545766238 ecl:hzl\n\
                     eyr:2022\n\
                     \n\
                     iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(part2(&input_generator(input)), 4);
    }
}
