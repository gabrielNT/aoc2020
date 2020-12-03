pub struct Map {
    map_vec: Vec<Vec<char>>,
    height: usize,
    width: usize,   // Total width
    t_width: usize, // Width of the original tile
}

impl Map {
    fn get_idx(&mut self, h: usize, w: usize) -> Option<char> {
        if h > self.height {
            return None;
        }

        while w >= self.width {
            self.expand_tile();
        }
        Some(self.map_vec[h][w])
    }

    fn expand_tile(&mut self) {
        for i in self.map_vec.iter_mut() {
            i.resize(self.width + self.t_width, '0');
            i.copy_within(0..self.t_width, self.width);
        }
        self.width = self.width + self.t_width;
    }
}

// I want this mutable, not sure how to do with the macro
pub fn input_generator(input: &str) -> Map {
    let map_vec_int: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    Map {
        height: map_vec_int.len(),
        width: map_vec_int[0].len(),
        t_width: map_vec_int[0].len(),
        map_vec: map_vec_int,
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut input = input_generator(input);
    count_trees_hit(&mut input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut input = input_generator(input);
    let mut trees_mult = 1;

    trees_mult = trees_mult * count_trees_hit(&mut input, 1, 1);
    trees_mult = trees_mult * count_trees_hit(&mut input, 3, 1);
    trees_mult = trees_mult * count_trees_hit(&mut input, 5, 1);
    trees_mult = trees_mult * count_trees_hit(&mut input, 7, 1);
    trees_mult = trees_mult * count_trees_hit(&mut input, 1, 2);
    trees_mult
}

// Helper
fn count_trees_hit(input: &mut Map, right: usize, down: usize) -> usize {
    let mut h_idx = 0;
    let mut w_idx = 0;
    let mut tree_count = 0;

    while h_idx < input.height - 1 {
        h_idx += down;
        w_idx += right;

        match input.get_idx(h_idx, w_idx) {
            None => (),
            Some('#') => tree_count += 1,
            _ => (),
        }
    }

    tree_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut map_vec: Vec<Vec<char>> = Vec::new();
        map_vec.push(vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.']);
        map_vec.push(vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.']);

        assert_eq!(input_generator("..##.......\n#...#...#..").map_vec, map_vec);
        assert_eq!(input_generator("..##.......\n#...#...#..").height, 2);
        assert_eq!(input_generator("..##.......\n#...#...#..").width, 11);
    }

    #[test]
    fn test_expand() {
        let mut map_vec: Vec<Vec<char>> = Vec::new();
        map_vec.push(vec![
            '.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '#', '.', '.',
            '.', '.', '.', '.', '.',
        ]);
        map_vec.push(vec![
            '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '.',
            '.', '.', '#', '.', '.',
        ]);

        let mut input = input_generator("..##.......\n#...#...#..");
        input.expand_tile();
        assert_eq!(input.map_vec, map_vec);
        assert_eq!(input.width, 22);
        assert_eq!(input.t_width, 11);
    }

    #[test]
    fn test_idx() {
        let mut input = input_generator("..##.......\n#...#...#..");
        assert_eq!(input.get_idx(1, 1), Some('.'));
    }

    #[test]
    fn test_idx_expand() {
        let mut input = input_generator("..##.......\n#...#...#..");
        assert_eq!(input.get_idx(0, 33), Some('.'));
    }

    #[test]
    fn test_part1() {
        let input = "..##.........##.........##.........##.........##.........##.......\n\
                     #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..\n\
                     .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.\n\
                     ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#\n\
                     .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.\n\
                     ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....\n\
                     .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#\n\
                     .#........#.#........#.#........#.#........#.#........#.#........#\n\
                     #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...\n\
                     #...##....##...##....##...##....##...##....##...##....##...##....#\n\
                     .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = "..##.........##.........##.........##.........##.........##.......\n\
                     #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..\n\
                     .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.\n\
                     ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#\n\
                     .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.\n\
                     ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....\n\
                     .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#\n\
                     .#........#.#........#.#........#.#........#.#........#.#........#\n\
                     #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...\n\
                     #...##....##...##....##...##....##...##....##...##....##...##....#\n\
                     .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";
        assert_eq!(part2(input), 336);
    }
}
