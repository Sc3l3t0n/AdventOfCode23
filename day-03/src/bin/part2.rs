use std::collections::HashSet;

fn main() {
    println!("Part 2");
    let input = include_str!("./input2.txt");
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

struct Gear {
    pub line: usize,
    pub at: usize,
}

impl Gear {
    pub fn get_gear_rotation(&self, numbers: &[Vec<Option<u32>>]) -> Option<u32> {
        let mut set: HashSet<u32> = HashSet::new();

        let start = self.at.saturating_sub(1);
        // Check if line above is valid
        if self.line != 0 {
            for i in start..self.at + 2 {
                // Check if corner is valid
                if i + 1 > numbers[self.line].len() {
                    continue;
                }
                if let Some(number) = numbers[self.line - 1][i] {
                    set.insert(number);
                }
            }
        }
        // Check if line under is valid
        if self.line < numbers.len() - 1 {
            for i in start..self.at + 2 {
                // Check if corner is valid
                if i + 1 > numbers[self.line].len() {
                    continue;
                }
                if let Some(number) = numbers[self.line + 1][i] {
                    set.insert(number);
                }
            }
        }
        // Check if edge is valid (left)
        if self.at > 0 {
            if let Some(number) = numbers[self.line][self.at - 1] {
                set.insert(number);
            }
        }
        // Check if edge is valid (right)
        if self.at < numbers[self.line].len() - 1 {
            if let Some(number) = numbers[self.line][self.at + 1] {
                set.insert(number);
            }
        }
        if set.len() == 2 {
            let mut iter = set.iter();
            match (iter.next(), iter.next()) {
                (Some(&first), Some(&second)) => Some(first * second),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn solve(input: &str) -> String {
    let chars = convert_to_char_matrix(input);
    let numbers = find_numbers(&chars);
    let number_grid = create_number_grid(&numbers, (chars[0].len(), chars.len()));
    let gears = find_gears(&chars);
    add_gear_ratios(&gears, &number_grid).to_string()
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

fn find_gears(chars: &[Vec<char>]) -> Vec<Gear> {
    let mut result: Vec<_> = Vec::new();
    for line in chars.iter().enumerate() {
        for char in line.1.iter().enumerate() {
            if *char.1 == '*' {
                result.insert(
                    result.len(),
                    Gear {
                        line: line.0,
                        at: char.0,
                    },
                );
            }
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

fn create_number_grid(numbers: &[Number], size: (usize, usize)) -> Vec<Vec<Option<u32>>> {
    let mut result: Vec<_> = Vec::new();
    for i in 0..size.1 {
        result.insert(i, Vec::new());
        for j in 0..size.0 {
            result[i].insert(j, None);
        }
    }
    for number in numbers {
        for i in number.start..number.end + 1 {
            result[number.line][i] = Some(number.number);
        }
    }
    result
}

fn add_gear_ratios(gears: &[Gear], numbers: &[Vec<Option<u32>>]) -> u32 {
    let mut result = 0;
    for gear in gears {
        if let Some(rotation) = gear.get_gear_rotation(numbers) {
            result += rotation;
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
        assert_eq!(output, "467835".to_string());
    }
}

