#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}
#[derive(Debug, PartialEq)]
pub struct Instruction {
    direction: Direction,
    units: i32,
}
#[derive(Debug, PartialEq)]
pub struct Ship {
    instructions: Vec<Instruction>,
    curr_direction: Direction,
    curr_position: (i32, i32), //north-south, east-west
    waypoint: (i32, i32),
    waypoint_directions: (Direction, Direction),
}

impl Ship {
    pub fn process_instructions(&mut self) {
        for i in self.instructions.iter() {
            match i.direction {
                Direction::North => self.curr_position.0 += i.units,
                Direction::South => self.curr_position.0 -= i.units,
                Direction::East => self.curr_position.1 += i.units,
                Direction::West => self.curr_position.1 -= i.units,
                Direction::Left => {
                    self.curr_direction =
                        Ship::change_direction(&self.curr_direction, Direction::Left, i.units)
                }
                Direction::Right => {
                    self.curr_direction =
                        Ship::change_direction(&self.curr_direction, Direction::Right, i.units)
                }
                Direction::Forward => match self.curr_direction {
                    Direction::North => self.curr_position.0 += i.units,
                    Direction::South => self.curr_position.0 -= i.units,
                    Direction::East => self.curr_position.1 += i.units,
                    Direction::West => self.curr_position.1 -= i.units,
                    _ => panic!("Invalid current direction"),
                },
            }
        }
    }

    pub fn change_direction(
        original_direction: &Direction,
        turn_direction: Direction,
        units: i32,
    ) -> Direction {
        match turn_direction {
            Direction::Left => {
                return Ship::turn_left(original_direction, units);
            }
            Direction::Right => {
                return Ship::turn_right(original_direction, units);
            }
            _ => panic!("Only L and R change directions!"),
        }
    }

    pub fn process_instructions2(&mut self) {
        for i in self.instructions.iter() {
            match i.direction {
                Direction::North => {
                    match self.waypoint_directions.0 {
                        Direction::North => self.waypoint.0 += i.units,
                        Direction::South => self.waypoint.0 -= i.units,
                        _ => (),
                    }
                    match self.waypoint_directions.1 {
                        Direction::North => self.waypoint.1 += i.units,
                        Direction::South => self.waypoint.1 -= i.units,
                        _ => (),
                    }
                }
                Direction::South => {
                    match self.waypoint_directions.0 {
                        Direction::North => self.waypoint.0 -= i.units,
                        Direction::South => self.waypoint.0 += i.units,
                        _ => (),
                    }
                    match self.waypoint_directions.1 {
                        Direction::North => self.waypoint.1 -= i.units,
                        Direction::South => self.waypoint.1 += i.units,
                        _ => (),
                    }
                }
                Direction::East => {
                    match self.waypoint_directions.0 {
                        Direction::East => self.waypoint.0 += i.units,
                        Direction::West => self.waypoint.0 -= i.units,
                        _ => (),
                    }
                    match self.waypoint_directions.1 {
                        Direction::East => self.waypoint.1 += i.units,
                        Direction::West => self.waypoint.1 -= i.units,
                        _ => (),
                    }
                }
                Direction::West => {
                    match self.waypoint_directions.0 {
                        Direction::East => self.waypoint.0 -= i.units,
                        Direction::West => self.waypoint.0 += i.units,
                        _ => (),
                    }
                    match self.waypoint_directions.1 {
                        Direction::East => self.waypoint.1 -= i.units,
                        Direction::West => self.waypoint.1 += i.units,
                        _ => (),
                    }
                }
                Direction::Left => {
                    self.waypoint_directions.0 = Ship::change_direction(
                        &self.waypoint_directions.0,
                        Direction::Left,
                        i.units,
                    );
                    self.waypoint_directions.1 = Ship::change_direction(
                        &self.waypoint_directions.1,
                        Direction::Left,
                        i.units,
                    );
                }
                Direction::Right => {
                    self.waypoint_directions.0 = Ship::change_direction(
                        &self.waypoint_directions.0,
                        Direction::Right,
                        i.units,
                    );
                    self.waypoint_directions.1 = Ship::change_direction(
                        &self.waypoint_directions.1,
                        Direction::Right,
                        i.units,
                    );
                }
                Direction::Forward => {
                    for _ in 0..i.units {
                        match self.waypoint_directions.0 {
                            Direction::North => self.curr_position.0 += self.waypoint.0,
                            Direction::South => self.curr_position.0 -= self.waypoint.0,
                            Direction::East => self.curr_position.1 += self.waypoint.0,
                            Direction::West => self.curr_position.1 -= self.waypoint.0,
                            _ => panic!("Waypoint direction 0 should be in {N, S, E, W}"),
                        }
                        match self.waypoint_directions.1 {
                            Direction::North => self.curr_position.0 += self.waypoint.1,
                            Direction::South => self.curr_position.0 -= self.waypoint.1,
                            Direction::East => self.curr_position.1 += self.waypoint.1,
                            Direction::West => self.curr_position.1 -= self.waypoint.1,
                            _ => panic!("Waypoint direction 1 should be in {N, S, E, W}"),
                        }
                    }
                }
            }
        }
    }

    fn turn_left(original_direction: &Direction, units: i32) -> Direction {
        match original_direction {
            Direction::North => {
                if units == 90 {
                    return Direction::West;
                } else if units == 180 {
                    return Direction::South;
                } else if units == 270 {
                    return Direction::East;
                }
            }
            Direction::South => {
                if units == 90 {
                    return Direction::East;
                } else if units == 180 {
                    return Direction::North;
                } else if units == 270 {
                    return Direction::West;
                }
            }
            Direction::East => {
                if units == 90 {
                    return Direction::North;
                } else if units == 180 {
                    return Direction::West;
                } else if units == 270 {
                    return Direction::South;
                }
            }
            Direction::West => {
                if units == 90 {
                    return Direction::South;
                } else if units == 180 {
                    return Direction::East;
                } else if units == 270 {
                    return Direction::North;
                }
            }
            _ => panic!("curr_direction needs to be one of {N, S, E, W}"),
        }
        panic!("Turn left failed");
    }

    fn turn_right(original_direction: &Direction, units: i32) -> Direction {
        match original_direction {
            Direction::North => {
                if units == 90 {
                    return Direction::East;
                } else if units == 180 {
                    return Direction::South;
                } else if units == 270 {
                    return Direction::West;
                }
            }
            Direction::South => {
                if units == 90 {
                    return Direction::West;
                } else if units == 180 {
                    return Direction::North;
                } else if units == 270 {
                    return Direction::East;
                }
            }
            Direction::East => {
                if units == 90 {
                    return Direction::South;
                } else if units == 180 {
                    return Direction::West;
                } else if units == 270 {
                    return Direction::North;
                }
            }
            Direction::West => {
                if units == 90 {
                    return Direction::North;
                } else if units == 180 {
                    return Direction::East;
                } else if units == 270 {
                    return Direction::South;
                }
            }
            _ => panic!("curr_direction needs to be one of {N, S, E, W}"),
        }
        panic!("Turn left failed");
    }
}

pub fn input_generator(input: &str) -> Ship {
    let instructions = input
        .lines()
        .map(|l| match l.chars().next().unwrap() {
            'N' => Instruction {
                direction: Direction::North,
                units: l[1..].parse::<i32>().unwrap(),
            },
            'S' => Instruction {
                direction: Direction::South,
                units: l[1..].parse::<i32>().unwrap(),
            },
            'E' => Instruction {
                direction: Direction::East,
                units: l[1..].parse::<i32>().unwrap(),
            },
            'W' => Instruction {
                direction: Direction::West,
                units: l[1..].parse::<i32>().unwrap(),
            },
            'L' => Instruction {
                direction: Direction::Left,
                units: l[1..].parse::<i32>().unwrap(),
            },
            'R' => Instruction {
                direction: Direction::Right,
                units: l[1..].parse::<i32>().unwrap(),
            },
            'F' => Instruction {
                direction: Direction::Forward,
                units: l[1..].parse::<i32>().unwrap(),
            },
            _ => panic!("Invalid input!"),
        })
        .collect();

    Ship {
        instructions,
        curr_direction: Direction::East,
        curr_position: (0, 0),
        waypoint: (1, 10),
        waypoint_directions: (Direction::North, Direction::East),
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut ship = input_generator(input);
    ship.process_instructions();

    (ship.curr_position.0.abs() + ship.curr_position.1.abs()) as usize
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut ship = input_generator(input);
    ship.process_instructions2();

    (ship.curr_position.0.abs() + ship.curr_position.1.abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            input_generator("F10\nN3"),
            Ship {
                instructions: vec![
                    Instruction {
                        direction: Direction::Forward,
                        units: 10
                    },
                    Instruction {
                        direction: Direction::North,
                        units: 3
                    }
                ],
                curr_direction: Direction::East,
                curr_position: (0, 0),
                waypoint: (1, 10),
                waypoint_directions: (Direction::North, Direction::East),
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("F10\nN3\nF7\nR90\nF11"), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("F10\nN3\nF7\nR90\nF11"), 286);
        assert_eq!(
            part2(
                "N3\nL90\nF63\nW5\nF46\nE3\nF22\nN2\nR90\nF68\nE4\nW3\nR90\nW4\nR180\nE1\nS5\nF90"
            ),
            2510
        );
    }
}
