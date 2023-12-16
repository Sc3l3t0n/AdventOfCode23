use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u32},
    combinator::map,
    multi::many1,
    multi::separated_list1,
    IResult,
};

fn main() {
    println!("Part 1");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

fn solve(input: &str) -> String {
    let _rows = parse_springs_list(input).unwrap().1;
    String::from(input)
}

struct Row {
    springs: Vec<Springs>,
    broken: Vec<usize>,
    unknown: Vec<Range<usize>>,
}

impl Row {
    fn get_posibilities_of_broken(&self) -> usize {
        let mut posibilities = 0;

        posibilities
    }

    fn gen_unknown(springs: &Vec<Springs>) -> Vec<Range<usize>> {
        let mut unknown = vec![];
        let mut start = None;
        for (i, spring) in springs.iter().enumerate() {
            match spring {
                Springs::Unknown => {
                    if start.is_none() {
                        start = Some(i);
                    }
                }
                _ => {
                    if let Some(start) = start {
                        unknown.push(start..i);
                    }
                    start = None;
                }
            }
        }
        if let Some(start) = start {
            unknown.push(start..springs.len());
        }
        unknown
    }
}

// Parser

enum Springs {
    Unknown,
    Damaged,
    Operational,
}

fn parse_spring(input: &str) -> IResult<&str, Springs> {
    alt((
        map(char('.'), |_| Springs::Operational),
        map(char('#'), |_| Springs::Damaged),
        map(char('?'), |_| Springs::Unknown),
    ))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, numbers) = separated_list1(char(','), u32)(input)?;
    let numbers = numbers.into_iter().map(|n| n as usize).collect();
    Ok((input, numbers))
}

fn parse_springs_row(input: &str) -> IResult<&str, Row> {
    let (input, springs) = many1(parse_spring)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, broken) = parse_numbers(input)?;

    let (input, _) = alt((tag("\r\n"), tag("\n")))(input)?;

    let unknown = Row::gen_unknown(&springs);

    let row = Row {
        springs,
        broken,
        unknown,
    };

    Ok((input, row))
}

fn parse_springs_list(input: &str) -> IResult<&str, Vec<Row>> {
    many1(parse_springs_row)(input)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let output = solve(input);
        assert_eq!(output, "21".to_string());
    }
}

