use std::collections::HashMap;

fn main() {
    let possible_pouch = Pouch {
        red_cubes: 12,
        green_cubes: 13,
        blue_cubes: 14,
    };
    println!("Part 1");
    let result = solve(include_str!("./input1.txt"), possible_pouch);
    println!("Result = {}", result);
}

struct Pouch {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

struct Draw {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl Pouch {
    pub fn is_possible(
        &self,
        Draw {
            red_cubes,
            green_cubes,
            blue_cubes,
        }: Draw,
    ) -> bool {
        self.red_cubes >= red_cubes
            && self.green_cubes >= green_cubes
            && self.blue_cubes >= blue_cubes
    }
}

impl Draw {
    pub fn from_str(input: &str) -> Self {
        let mut red_cubes: u32 = 0;
        let mut green_cubes: u32 = 0;
        let mut blue_cubes: u32 = 0;

        for draw in input.split(';') {
            for cube in draw.split(',') {
                let splitted: Vec<&str> = cube.trim().split(' ').collect();
                let number: u32 = splitted[0].parse().unwrap();
                if splitted[1] == "red" && number > red_cubes {
                    red_cubes = number;
                } else if splitted[1] == "green" && number > green_cubes {
                    green_cubes = number;
                } else if splitted[1] == "blue" && number > blue_cubes {
                    blue_cubes = number;
                }
            }
        }
        Self {
            red_cubes,
            green_cubes,
            blue_cubes,
        }
    }
}

fn solve(input: &str, possible_pouch: Pouch) -> String {
    let game_lines = convert_game_lines(input);
    process_game_lines(game_lines, possible_pouch).to_string()
}

fn process_game_lines(map: HashMap<u32, &str>, possible_pouch: Pouch) -> u32 {
    let mut result: u32 = 0;
    for game in map {
        let draw = Draw::from_str(game.1);
        if possible_pouch.is_possible(draw) {
            result += game.0;
        }
    }
    result
}

fn convert_game_lines(game_lines: &str) -> HashMap<u32, &str> {
    let mut map = HashMap::new();

    for line in game_lines.lines() {
        if line.is_empty() {
            continue;
        }
        let splitted: Vec<&str> = line.split(':').collect();
        let game_number: u32 = splitted[0].split_at(5).1.parse().unwrap();
        map.insert(game_number, splitted[1]);
    }

    map
}

#[cfg(test)]
mod tests {
    use crate::{solve, Pouch};

    #[test]
    fn example_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let possible_pouch = Pouch {
            red_cubes: 12,
            green_cubes: 13,
            blue_cubes: 14,
        };
        let output = solve(input, possible_pouch);
        assert_eq!(output, "8".to_string());
    }
}

