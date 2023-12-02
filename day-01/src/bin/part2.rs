fn main() {
    println!("Part 2");

    let input = include_str!("./input2.txt");
    let output = parse_input(input);
    println!("{output}");
}

fn parse_input(input: &str) -> String {
    let mut result: u32 = 0;
    for line in input.lines() {
        result += get_sum_first_last(line);
    }

    result.to_string()
}

fn get_sum_first_last(line: &str) -> u32 {
    let word_numbers = get_first_last_number_of_words_index(line);
    let digit_numbers = get_first_last_number_index(line);

    let mut vec = Vec::new();

    vec.extend(word_numbers.0);
    vec.extend(word_numbers.1);
    vec.extend(digit_numbers.0);
    vec.extend(digit_numbers.1);

    vec.sort_unstable_by_key(|x| x.0);

    // get first and last and sum
    let mut sum = 0;
    if let Some(first) = vec.first() {
        sum += first.1 * 10;
    };
    if let Some(last) = vec.last() {
        sum += last.1;
    };
    sum
}

fn convert_number(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn get_first_last_number_of_words_index(line: &str) -> (Option<(usize, u32)>, Option<(usize, u32)>) {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result = Vec::new();

    for number in numbers {
        result.append(&mut line.match_indices(number).collect());
    }

    result.sort_unstable_by_key(|x| x.0);

    let mut tupel: (Option<(usize, u32)>, Option<(usize, u32)>) = (None, None);

    if let Some(first) = result.first() {
        tupel.0 = Some((first.0, convert_number(first.1)));
    }
    if let Some(last) = result.last() {
        tupel.1 = Some((last.0, convert_number(last.1)));
    }

    tupel
}

fn get_first_last_number_index(line: &str) -> (Option<(usize, u32)>, Option<(usize, u32)>) {
    let mut first_digit: Option<(usize, u32)> = None;
    let mut last_digit: Option<(usize, u32)> = None;
    
    for (index, character) in line.char_indices() {
        if let Some(digit) = character.to_digit(10) {
            last_digit = Some((index, digit));
            if first_digit.is_none() {
                first_digit = Some((index, digit));
            }
        }
    }

    (first_digit, last_digit)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn example_input() {
        let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let output = parse_input(input);
        assert_eq!(output, "281".to_string());
    }
}

