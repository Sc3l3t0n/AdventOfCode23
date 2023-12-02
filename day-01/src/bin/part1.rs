fn main() {
    println!("Part 1");

    let input = include_str!("./input1.txt");
    let output = parse_input(input);
    println!("{output}");
}

fn parse_input(input: &str) -> String {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut result: u32 = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        result += digits.first().clone().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0);
    }
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_input() {
        let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let output = parse_input(input);
        assert_eq!(output, "142".to_string());
    }
}

