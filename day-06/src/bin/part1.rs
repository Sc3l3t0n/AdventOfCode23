fn main() {
    println!("Part 1");
    let input = include_str!("../../input1.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

struct Game {
    time: u32,
    record: u32,
}

fn solve(input: &str) -> String {
    let games = parse_games(input);
    get_possibilities_combined(&games).to_string()
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut result = Vec::new();
    let mut lines = input.lines();
    let mut times = lines
        .next()
        .expect("Not in the right format")
        .split_ascii_whitespace()
        .skip(1);
    let mut distances = lines
        .next()
        .expect("Not in the right format")
        .split_ascii_whitespace()
        .skip(1);

    while let (Some(time), Some(distance)) = (times.next(), distances.next()) {
        result.push(Game {
            time: time.parse().expect("Should be a number"),
            record: distance.parse().expect("Should be a number"),
        })
    }
    result
}

fn possibilities_break_record(Game { time, record }: &Game) -> u32 {
    let mut poss = 0;
    let mut passed = false;
    for i in 0..*time {
        let speed = i;
        let left = time - i;
        if speed * left > *record {
            poss += 1;
            passed = true;
        } else if passed {
            break;
        }
    }
    poss
}

fn get_possibilities_combined(games: &[Game]) -> u32 {
    let mut result = 1;
    for game in games {
        result *= possibilities_break_record(game);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let output = solve(input);
        assert_eq!(output, "288".to_string());
    }
}

