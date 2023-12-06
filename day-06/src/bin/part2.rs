use std::str::Lines;

fn main() {
    println!("Part 2");
    let input = include_str!("../../input2.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Default)]
struct Game {
    time: u64,
    record: u64,
}

fn solve(input: &str) -> String {
    let games = parse_games(input);
    possibilities_break_record(&games).to_string()
}

fn parse_games(input: &str) -> Game {
    let mut lines = input.lines();
    let time: u64 = parse_big_number(&mut lines);
    let record: u64 = parse_big_number(&mut lines);

    Game { time, record }
}

fn parse_big_number(input: &mut Lines) -> u64 {
    String::from(
        input
            .next()
            .expect("Not in the right format")
            .split(':')
            .skip(1)
            .collect::<Vec<_>>()[0],
    )
    .replace(' ', "")
    .parse()
    .expect("Should be a number")
}

fn possibilities_break_record(Game { time, record }: &Game) -> u64 {
    let mut poss = 0;
    let mut passed = false;
    for i in 0..*time + 1 {
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

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        let output = solve(input);
        assert_eq!(output, "71503".to_string());
    }
}

