use std::collections::HashMap;

fn main() {
    println!("Part 1");
    let input = include_str!("./input1.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

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

struct Game {
    nodes: HashMap<String, (String, String)>,
    current_node: String,
    final_node: String,
    directions: Roundabout<Direction>,
    steps: usize,
}

impl Game {
    fn run(&mut self) {
        while !self.current_node.eq(&self.final_node) {
            self.next_node();
        }
    }
    fn next_node(&mut self) {
        let (left, right) = self.nodes.get(&self.current_node).unwrap();
        let direction = self.directions.next().unwrap();
        match direction {
            Direction::Left => self.current_node = left.clone(),
            Direction::Right => self.current_node = right.clone(),
        }
        self.steps += 1;
    }
}

fn solve(input: &str) -> String {
    let mut game = parse_input(input);
    game.run();
    game.steps.to_string()
}

fn parse_input(input: &str) -> Game {
    let mut nodes = HashMap::new();
    let mut starts = Vec::new();
    let mut directions = Roundabout::new(Vec::new());
    let steps = 0;

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    for c in first_line.chars() {
        directions.add(Direction::parse(c));
    }
    lines.next(); // skip blank line
    for line in lines {
        let parts: Vec<&str> = line.split('=').collect();
        let start = parts[0].trim().to_string();
        let mut node = parts[1].split(',');
        let left = node
            .next()
            .expect("Should be in right format")
            .replace('(', " ")
            .trim()
            .to_string();
        let right = node
            .next()
            .expect("Should be in right format")
            .replace(')', " ")
            .trim()
            .to_string();
        starts.push(start.clone());
        nodes.insert(start, (left, right));
    }
    let current_node = String::from("AAA");
    let final_node = String::from("ZZZ");
    Game {
        nodes,
        current_node,
        final_node,
        directions,
        steps,
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        let output = solve(input);
        assert_eq!(output, "2".to_string());
    }

    #[test]
    fn example_input_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let output = solve(input);
        assert_eq!(output, "6".to_string());
    }
}

