use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

fn main() {
    println!("Part 1");
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
        let mut positive = true;
        for index in 1..self.rows.len() {
            for offset in 1..=usize::min(self.rows.len() - index, index) {
                let left = &self.rows[index - offset];
                let right = &self.rows[index + offset - 1];
                if left != right {
                    positive = false;
                    break;
                }
            }
            if positive {
                return Some(index);
            }
            positive = true;
        }
        None
    }

    fn get_vertical_mirror(&self) -> Option<usize> {
        let mut positive = true;
        for index in 1..self.columns.len() {
            for offset in 1..=usize::min(self.columns.len() - index, index) {
                let top = &self.columns[index - offset];
                let bottom = &self.columns[index + offset - 1];
                if top != bottom {
                    positive = false;
                    break;
                }
            }
            if positive {
                return Some(index);
            }
            positive = true;
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
        assert_eq!(output, "405".to_string());
    }
}

