fn main() {
    println!("Part 2");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

fn solve(input: &str) -> String {
    String::from(input)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "
";
        let output = solve(input);
        assert_eq!(output, "".to_string());
    }
}

