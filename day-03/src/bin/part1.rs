fn main() {
    println!("Part 1");
    let input = include_str!("./input1.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Clone, Copy)]
struct Number {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub number: u32,
}

impl Number {
    pub fn is_partnumber(&self, chars: &[Vec<char>]) -> bool {
        let start = self.start.saturating_sub(1);
        // Check if line above is valid
        if self.line != 0 {
            for i in start..self.end + 2 {
                // Check if corner is valid
                if i + 1 > chars[self.line].len() {
                    continue;
                }
                let ch = chars[self.line - 1][i];
                if !ch.is_ascii_digit() && ch != '.' {
                    return true;
                }
            }
        }
        // Check if line under is valid
        if self.line < chars.len() - 1 {
            for i in start..self.end + 2 {
                // Check if corner is valid
                if i + 1 > chars[self.line].len() {
                    continue;
                }
                let ch = chars[self.line + 1][i];
                if !ch.is_ascii_digit() && ch != '.' {
                    return true;
                }
            }
        }
        // Check if edge is valid (left)
        if self.start > 0 {
            let ch = chars[self.line][self.start - 1];
            if !ch.is_ascii_digit() && ch != '.' {
                return true;
            }
        }
        // Check if edge is valid (right)
        if self.end < chars[self.line].len() - 1 {
            let ch = chars[self.line][self.end + 1];
            if !ch.is_ascii_digit() && ch != '.' {
                return true;
            }
        }
        false
    }
}

fn solve(input: &str) -> String {
    let chars = convert_to_char_matrix(input);
    let numbers = find_numbers(&chars);
    add_part_numbers(&numbers, &chars).to_string()
}

fn convert_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in input.lines().enumerate() {
        result.insert(line.0, Vec::new());
        for char in line.1.char_indices() {
            result[line.0].insert(char.0, char.1);
        }
    }
    result
}

fn find_numbers(chars: &[Vec<char>]) -> Vec<Number> {
    let mut result: Vec<_> = Vec::new();
    let mut current_number: Option<Number> = None;
    for line in chars.iter().enumerate() {
        for char in line.1.iter().enumerate() {
            if char.1.is_ascii_digit() {
                if let Some(ref mut number) = current_number {
                    number.end = char.0;
                } else {
                    current_number = Some(Number {
                        line: line.0,
                        start: char.0,
                        end: char.0,
                        number: 0,
                    })
                }
            } else if let Some(ref mut number) = current_number {
                let string_number: String = line.1[number.start..number.end + 1].iter().collect();
                number.number = string_number.parse().unwrap();
                result.insert(result.len(), *number);
                current_number = None;
            }
        }
        if let Some(ref mut number) = current_number {
            let string_number: String = line.1[number.start..number.end + 1].iter().collect();
            number.number = string_number.parse().unwrap();
            result.insert(result.len(), *number);
            current_number = None;
        }
    }
    result
}

fn add_part_numbers(numbers: &[Number], chars: &[Vec<char>]) -> u32 {
    let mut result = 0;
    for number in numbers {
        if number.is_partnumber(chars) {
            result += number.number;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let output = solve(input);
        assert_eq!(output, "4361".to_string());
    }
}

