use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Cube {
    Active,
    Inactive,
}

pub struct Dimension {
    state_map: HashMap<(i32, i32, i32, i32), Cube>,
    neighbours_map: HashMap<(i32, i32, i32, i32), usize>,
}

impl Dimension {
    pub fn process_neighbours(&mut self) {
        // Clear everything on start
        self.neighbours_map = HashMap::new();

        // Compute the number of neighbours for all cells
        for (coord, state) in &self.state_map {
            // Increment in all 3 dimensions
            for i in -1..2 {
                for j in -1..2 {
                    for k in -1..2 {
                        if state == &Cube::Active {
                            if i == 0 && j == 0 && k == 0 {
                                continue;
                            }

                            let new_coord = (coord.0 + i, coord.1 + j, coord.2 + k, 0);
                            if !self.neighbours_map.contains_key(&new_coord) {
                                self.neighbours_map.insert(new_coord, 1);
                            } else {
                                self.neighbours_map
                                    .insert(new_coord, self.neighbours_map[&new_coord] + 1);
                            }
                        }
                    }
                }
            }
        }

        //Update state_map with the new cells
        let mut new_states = HashMap::new();
        for (coord, count) in &self.neighbours_map {
            if self.state_map.contains_key(&coord) {
                match self.state_map[&coord] {
                    Cube::Active => {
                        if count == &2 || count == &3 {
                            new_states.insert(*coord, Cube::Active);
                        }
                    }
                    Cube::Inactive => {
                        if count == &3 {
                            new_states.insert(*coord, Cube::Active);
                        }
                    }
                }
            } else {
                if count == &3 {
                    new_states.insert(*coord, Cube::Active);
                }
            }
        }

        self.state_map = new_states;
    }

    pub fn process_neighbours_4d(&mut self) {
        // Clear everything on start
        self.neighbours_map = HashMap::new();

        // Compute the number of neighbours for all cells
        for (coord, state) in &self.state_map {
            // Increment in all 3 dimensions
            for i in -1..2 {
                for j in -1..2 {
                    for k in -1..2 {
                        for l in -1..2 {
                            if state == &Cube::Active {
                                if i == 0 && j == 0 && k == 0 && l == 0 {
                                    continue;
                                }

                                let new_coord =
                                    (coord.0 + i, coord.1 + j, coord.2 + k, coord.3 + l);
                                if !self.neighbours_map.contains_key(&new_coord) {
                                    self.neighbours_map.insert(new_coord, 1);
                                } else {
                                    self.neighbours_map
                                        .insert(new_coord, self.neighbours_map[&new_coord] + 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        //Update state_map with the new cells
        let mut new_states = HashMap::new();
        for (coord, count) in &self.neighbours_map {
            if self.state_map.contains_key(&coord) {
                match self.state_map[&coord] {
                    Cube::Active => {
                        if count == &2 || count == &3 {
                            new_states.insert(*coord, Cube::Active);
                        }
                    }
                    Cube::Inactive => {
                        if count == &3 {
                            new_states.insert(*coord, Cube::Active);
                        }
                    }
                }
            } else {
                if count == &3 {
                    new_states.insert(*coord, Cube::Active);
                }
            }
        }

        self.state_map = new_states;
    }

    pub fn count_actives(&self) -> usize {
        let mut count = 0;

        for (_, state) in &self.state_map {
            if state == &Cube::Active {
                count += 1;
            }
        }

        count
    }
}

pub fn input_generator(input: &str) -> Dimension {
    let mut state_map = HashMap::new();

    for (x, l) in input.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            match c {
                '.' => {
                    state_map.insert((x as i32, y as i32, 0 as i32, 0 as i32), Cube::Inactive);
                }
                '#' => {
                    state_map.insert((x as i32, y as i32, 0 as i32, 0 as i32), Cube::Active);
                }
                _ => panic!("Invalid input!"),
            }
        }
    }

    Dimension {
        state_map,
        neighbours_map: HashMap::new(),
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let mut dimension = input_generator(input);

    for _ in 0..6 {
        dimension.process_neighbours();
    }

    dimension.count_actives()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let mut dimension = input_generator(input);

    for _ in 0..6 {
        dimension.process_neighbours_4d();
    }

    dimension.count_actives()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(input_generator(".#.\n..#\n###").state_map.len(), 9);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(".#.\n..#\n###"), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(".#.\n..#\n###"), 848);
    }
}
