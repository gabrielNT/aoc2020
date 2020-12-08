use regex::Regex;
use std::collections::HashMap;

pub struct Graph {
    adj_matrix: Vec<Vec<usize>>,
    keys_map: HashMap<String, usize>,
    node_count: usize,
    edge_count: usize,
    visited: Vec<bool>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj_matrix: Vec::new(),
            keys_map: HashMap::new(),
            node_count: 0,
            edge_count: 0,
            visited: Vec::new(),
        }
    }

    pub fn add_or_return_node(&mut self, value: &str) -> usize {
        if self.keys_map.contains_key(&value.to_string()) {
            return self.keys_map.get(&value.to_string()).unwrap().clone();
        } else {
            // Add new node to the adj matrix
            self.node_count += 1;

            let mut node_line = Vec::new();
            for n in self.adj_matrix.iter_mut() {
                n.push(0);
            }
            for _ in 0..self.node_count {
                node_line.push(0 as usize);
            }

            self.adj_matrix.push(node_line);
            self.keys_map.insert(value.to_string(), self.node_count - 1);
        }

        self.node_count - 1
    }

    pub fn add_edge(&mut self, origin: usize, destination: usize, weight: usize) {
        self.edge_count += 1;
        self.adj_matrix[origin][destination] = weight;
    }

    // This is terrible lol
    pub fn count_back_reachable(&mut self, start: usize, weight: usize) -> usize {
        if weight == 0 {
            self.visited = vec![false; self.node_count];
        }
        self.visited[start] = true;
        let mut reachable = weight;

        for i in 0..self.node_count {
            if self.adj_matrix[i][start] > 0 && !self.visited[i] {
                reachable += self.count_back_reachable(i, 1);
            }
        }

        reachable
    }

    pub fn count_total_reachable(&mut self, start: usize, first: bool) -> usize {
        let mut reachable = 1;
        if first {
            reachable = 0;
        }

        for i in 0..self.node_count {
            if self.adj_matrix[start][i] > 0 {
                reachable =
                    reachable + (self.adj_matrix[start][i] * self.count_total_reachable(i, false));
            }
        }

        reachable
    }
}

pub fn input_generator(input: &str) -> Graph {
    let mut graph = Graph::new();
    let origin_regex = Regex::new(r"(.*) bags contain (\d+.*)").unwrap();
    let contains_regex = Regex::new(r"(\d+) (.*) bag[s]*").unwrap();
    let empty_regex = Regex::new(r".*contain no other bags").unwrap();

    for l in input.lines() {
        if empty_regex.is_match(l) {
            continue;
        }

        let original_captures = &origin_regex.captures(l).unwrap();
        let contains_str = &original_captures[2];
        let origin_str = &original_captures[1];
        let origin_key = graph.add_or_return_node(origin_str);

        for b in contains_str.split(",") {
            let contained_captures = &contains_regex.captures(b).unwrap();
            let contained_count = contained_captures[1].parse::<usize>().unwrap();
            let destination_key = graph.add_or_return_node(&contained_captures[2]);
            graph.add_edge(origin_key, destination_key, contained_count);
        }
    }

    graph
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut graph = input_generator(input);
    let start = graph.add_or_return_node("shiny gold");
    graph.count_back_reachable(start, 0)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let mut graph = input_generator(input);
    let start = graph.add_or_return_node("shiny gold");
    graph.count_total_reachable(start, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let graph =
            input_generator("light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                                   dark orange bags contain 3 bright white bags, 4 muted yellow bags.");

        assert_eq!(graph.node_count, 4);
        assert_eq!(graph.edge_count, 4);
    }

    #[test]
    fn test_graph_index() {
        let mut graph = Graph::new();
        graph.add_or_return_node("test");
        assert_eq!(graph.add_or_return_node("test"), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                          dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                          bright white bags contain 1 shiny gold bag.\n\
                          muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                          shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                          dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                          vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                          faded blue bags contain no other bags.\n\
                          dotted black bags contain no other bags."
            ),
            4
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                          dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                          bright white bags contain 1 shiny gold bag.\n\
                          muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                          shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                          dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                          vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                          faded blue bags contain no other bags.\n\
                          dotted black bags contain no other bags."
            ),
            32
        );

        assert_eq!(
            part2(
                "shiny gold bags contain 2 dark red bags.\n\
                          dark red bags contain 2 dark orange bags.\n\
                          dark orange bags contain 2 dark yellow bags.\n\
                          dark yellow bags contain 2 dark green bags.\n\
                          dark green bags contain 2 dark blue bags.\n\
                          dark blue bags contain 2 dark violet bags.\n\
                          dark violet bags contain no other bags."
            ),
            126
        );
    }
}
