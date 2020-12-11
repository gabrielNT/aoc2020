#[derive(Debug, PartialEq, Clone)]
pub enum SeatStatus {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, PartialEq)]
pub struct Seats {
    layout: Vec<Vec<SeatStatus>>,
}

impl Seats {
    pub fn step1(&mut self) -> bool {
        let mut changed = false;
        let original = self.layout.clone();

        for i in 0..self.layout.len() {
            for j in 0..self.layout[i].len() {
                match self.layout[i][j] {
                    SeatStatus::Empty => {
                        if Seats::adjacent_occupied(&original, i, j) == 0 {
                            changed = true;
                            self.layout[i][j] = SeatStatus::Occupied;
                        }
                    }
                    SeatStatus::Occupied => {
                        if Seats::adjacent_occupied(&original, i, j) >= 4 {
                            changed = true;
                            self.layout[i][j] = SeatStatus::Empty;
                        }
                    }
                    SeatStatus::Floor => (),
                }
            }
        }

        changed
    }

    pub fn step2(&mut self) -> bool {
        let mut changed = false;
        let original = self.layout.clone();

        for i in 0..self.layout.len() {
            for j in 0..self.layout[i].len() {
                match self.layout[i][j] {
                    SeatStatus::Empty => {
                        if Seats::direction_occupied(&original, i, j) == 0 {
                            changed = true;
                            self.layout[i][j] = SeatStatus::Occupied;
                        }
                    }
                    SeatStatus::Occupied => {
                        if Seats::direction_occupied(&original, i, j) >= 5 {
                            changed = true;
                            self.layout[i][j] = SeatStatus::Empty;
                        }
                    }
                    SeatStatus::Floor => (),
                }
            }
        }

        changed
    }

    fn adjacent_occupied(layout: &Vec<Vec<SeatStatus>>, i: usize, j: usize) -> usize {
        let moves = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count: usize = 0;

        for m in moves {
            let new_i = i as i32 + m.0;
            let new_j = j as i32 + m.1;

            if new_i >= 0
                && (new_i as usize) < layout.len()
                && new_j >= 0
                && (new_j as usize) < layout[0].len()
                && layout[new_i as usize][new_j as usize] == SeatStatus::Occupied
            {
                count += 1;
            }
        }

        count
    }

    fn direction_occupied(layout: &Vec<Vec<SeatStatus>>, i: usize, j: usize) -> usize {
        let moves = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count: usize = 0;

        for m in moves {
            let mut new_i = i as i32 + m.0;
            let mut new_j = j as i32 + m.1;

            loop {
                if new_i < 0
                    || (new_i as usize) >= layout.len()
                    || new_j < 0
                    || (new_j as usize) >= layout[0].len()
                {
                    break;
                } else if layout[new_i as usize][new_j as usize] == SeatStatus::Occupied {
                    count += 1;
                    break;
                } else if layout[new_i as usize][new_j as usize] == SeatStatus::Empty {
                    break;
                }

                new_i += m.0;
                new_j += m.1;
            }
        }

        count
    }

    pub fn count_occupied(&self) -> usize {
        let mut count = 0;
        for row in self.layout.iter() {
            for seat in row.iter() {
                if seat == &SeatStatus::Occupied {
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn input_generator(input: &str) -> Seats {
    let layout = input
        .lines()
        .map(|l| {
            let mut row = Vec::new();

            for c in l.chars() {
                match c {
                    'L' => row.push(SeatStatus::Empty),
                    '#' => row.push(SeatStatus::Occupied),
                    '.' => row.push(SeatStatus::Floor),
                    _ => (),
                }
            }
            row
        })
        .collect();

    Seats { layout }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut seats = input_generator(input);
    while seats.step1() {
        continue;
    }

    seats.count_occupied()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut seats = input_generator(input);
    while seats.step2() {
        continue;
    }

    seats.count_occupied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator("#L.\nLL#"),
            Seats {
                layout: vec![
                    vec![SeatStatus::Occupied, SeatStatus::Empty, SeatStatus::Floor],
                    vec![SeatStatus::Empty, SeatStatus::Empty, SeatStatus::Occupied]
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL"), 
            37
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL"), 
            26
        );
    }
}
