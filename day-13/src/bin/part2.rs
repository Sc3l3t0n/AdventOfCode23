use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

fn main() {
    println!("Part 2");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

fn solve(input: &str) -> String {
    let fields = parse_fields(input).unwrap().1;
    let sum = get_sum_mirror(fields);
    sum.to_string()
}

fn get_sum_mirror(fields: Vec<Field>) -> usize {
    let mut sum = 0;
    for field in fields {
        if let Some(index) = field.get_horizontal_mirror() {
            sum += index * 100;
            continue;
        }
        if let Some(index) = field.get_vertical_mirror() {
            sum += index;
        }
    }
    sum
}

struct Field {
    rows: Vec<Vec<FieldType>>,
    columns: Vec<Vec<FieldType>>,
}

impl Field {
    fn get_horizontal_mirror(&self) -> Option<usize> {
        let mut error = 0;
        for index in 1..self.rows.len() {
            for offset in 1..=usize::min(self.rows.len() - index, index) {
                let left = &mut self.rows[index - offset].iter();
                let right = &mut self.rows[index + offset - 1].iter();
                while let (Some(&left), Some(&right)) = (left.next(), right.next()) {
                    if left != right {
                        error += 1;
                    }
                }
            }
            if error == 1 {
                return Some(index);
            }
            error = 0;
        }
        None
    }

    fn get_vertical_mirror(&self) -> Option<usize> {
        let mut error = 0;
        for index in 1..self.columns.len() {
            for offset in 1..=usize::min(self.columns.len() - index, index) {
                let top = &mut self.columns[index - offset].iter();
                let bottom = &mut self.columns[index + offset - 1].iter();
                while let (Some(&top), Some(&bottom)) = (top.next(), bottom.next()) {
                    if top != bottom {
                        error += 1;
                    }
                }
            }
            if error == 1 {
                return Some(index);
            }
            error = 0;
        }
        None
    }
}

// Parse input

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FieldType {
    Rock,
    Ash,
}

fn parse_row(input: &str) -> IResult<&str, Vec<FieldType>> {
    let (input, row) = many1(alt((
        map(char('#'), |_| FieldType::Rock),
        map(char('.'), |_| FieldType::Ash),
    )))(input)?;

    Ok((input, row))
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    let (input, rows) = separated_list1(line_ending, parse_row)(input)?;
    let mut columns = vec![Vec::new(); rows.first().unwrap().len()];
    for row in rows.iter() {
        for (x, field_type) in row.iter().enumerate() {
            columns[x].push(*field_type);
        }
    }
    Ok((input, Field { rows, columns }))
}

fn parse_fields(input: &str) -> IResult<&str, Vec<Field>> {
    separated_list1(pair(line_ending, line_ending), parse_field)(input)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let output = solve(input);
        assert_eq!(output, "400".to_string());
    }
}


