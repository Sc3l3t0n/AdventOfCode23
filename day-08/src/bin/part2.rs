use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    println!("Part 2");
    let input = include_str!("./input2.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(input: char) -> Direction {
        match input {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Roundabout<T> {
    data: Vec<T>,
    current_index: usize,
}

impl<T> Roundabout<T> {
    fn new(data: Vec<T>) -> Self {
        Self {
            data,
            current_index: 0,
        }
    }
    fn add(&mut self, item: T) {
        self.data.push(item);
    }

    fn next(&mut self) -> Option<&T> {
        let result = self.data.get(self.current_index);
        self.current_index += 1;
        if self.current_index == self.data.len() {
            self.current_index = 0;
        }
        result
    }
}

#[derive(Default)]
struct GameData {
    raw_strings: Vec<String>,
}

struct Game<'a> {
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
    current_nodes: Vec<&'a str>,
    directions: Roundabout<Direction>,
}

impl<'a> Game<'a> {
    fn run(&mut self) -> u64 {
        let mut iterations: Vec<u64> = Vec::new();
        for node in &self.current_nodes {
            iterations.push(self.get_iteration_len(node));
        }

        iterations.iter().fold(1u64, |prev, &x| lcm(prev, x))
    }
    fn get_iteration_len(&self, node: &str) -> u64 {
        let mut len = 0;
        let mut current_node = node;
        let mut directions = Roundabout::new(self.directions.data.clone());
        loop {
            len += 1;
            let &(left, right) = self.nodes.get(current_node).unwrap();
            current_node = match directions.next().unwrap() {
                Direction::Right => right,
                Direction::Left => left,
            };
            if current_node.ends_with('Z') {
                break;
            }
        }
        len
    }
}

fn solve(input: &str) -> String {
    let data = parse_data(input);
    let mut game = parse_input(&data);
    game.run().to_string()
}

fn parse_data(input: &str) -> GameData {
    let mut data = GameData::default();
    for line in input.lines() {
        data.raw_strings.push(line.trim().to_string());
    }
    data
}

fn parse_input(data: &GameData) -> Game {
    let mut nodes = HashMap::new();
    let mut directions = Roundabout::new(Vec::new());
    let mut current_nodes = Vec::new();

    let mut lines = data.raw_strings.iter();
    let first_line = lines.next().unwrap();
    for c in first_line.chars() {
        directions.add(Direction::parse(c));
    }
    lines.next(); // skip blank line
                  // NQH = (HPR, DTN)
    for line in lines {
        let start = &line[..3];

        let left = &line[7..10];
        let right = &line[12..15];

        if start.ends_with('A') {
            current_nodes.push(start);
        }
        nodes.insert(start, (left, right));
    }

    Game {
        nodes,
        current_nodes,
        directions,
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let output = solve(input);
        assert_eq!(output, "6".to_string());
    }
}

