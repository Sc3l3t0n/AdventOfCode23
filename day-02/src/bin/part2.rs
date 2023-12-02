fn main() {
    println!("Part 2");
    let result = solve(include_str!("./input2.txt"));
    println!("Result = {}", result);
}

struct Draw {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
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

    pub fn get_multiplied(&self) -> u32 {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

fn solve(input: &str) -> String {
    let game_lines = convert_game_lines(input);
    process_game_lines(game_lines).to_string()
}

fn process_game_lines(games: Vec<&str>) -> u32 {
    let mut result: u32 = 0;
    for game in games {
        let draw = Draw::from_str(game);
        result += draw.get_multiplied();
    }
    result
}

fn convert_game_lines(game_lines: &str) -> Vec<&str> {
    let mut games = Vec::new();

    for line in game_lines.lines() {
        if line.is_empty() {
            continue;
        }
        let splitted: Vec<&str> = line.split(':').collect();
        games.insert(games.len(), splitted[1]);
    }

    games
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let output = solve(input);
        assert_eq!(output, "2286".to_string());
    }
}

