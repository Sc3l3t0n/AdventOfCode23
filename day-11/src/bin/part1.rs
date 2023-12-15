use std::collections::HashSet;
use std::fmt;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::map, multi::many1,
    IResult,
};

fn main() {
    println!("Part 1");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

fn solve(input: &str) -> String {
    let mut space = Space::from(input);
    space.cosmic_expansion();
    // dbg!(&space.objects);
    // space.print();
    space.get_sum_smallest_distance().to_string()
}

// Logic

struct Space {
    objects: Vec<Vec<SpaceObject>>,
}

impl From<&str> for Space {
    fn from(input: &str) -> Self {
        let (_, objects) = parse_galaxy(input).unwrap();
        Self { objects }
    }
}

impl Space {
    fn gen_galaxy_index(&self) -> Vec<(usize, usize)> {
        let mut index = Vec::new();
        for (col, objects) in self.objects.iter().enumerate() {
            for (row, object) in objects.iter().enumerate() {
                if let SpaceObject::Galaxy = object {
                    index.push((row, col));
                }
            }
        }
        index
    }

    fn cosmic_expansion(&mut self) {
        // From 0 to size of objects
        let mut rows: HashSet<usize> = HashSet::from_iter(0..self.objects.first().unwrap().len());
        let mut cols: HashSet<usize> = HashSet::from_iter(0..self.objects.len());
        for (col, objects) in self.objects.iter().enumerate() {
            for (row, object) in objects.iter().enumerate() {
                if let SpaceObject::Galaxy = object {
                    rows.remove(&row);
                    cols.remove(&col);
                }
            }
        }
        let mut rows: Vec<usize> = Vec::from_iter(rows);
        let mut cols: Vec<usize> = Vec::from_iter(cols);
        rows.sort();
        cols.sort();
        // TODO genau umgekehrt
        for (offset, &row) in rows.iter().enumerate() {
            for galaxy_row in self.objects.iter_mut() {
                galaxy_row.insert(row + offset, SpaceObject::Empty);
            }
        }
        for (offset, &col) in cols.iter().enumerate() {
            self.objects.insert(
                col + offset,
                Vec::from_iter(
                    (0..self.objects.first().unwrap().len()).map(|_| SpaceObject::Empty),
                ),
            )
        }
    }

    fn get_sum_smallest_distance(&mut self) -> usize {
        let mut sum = 0;
        let mut galaxy_index = self.gen_galaxy_index();
        loop {
            if galaxy_index.is_empty() {
                break;
            }
            let start = galaxy_index.pop().unwrap();
            for end in galaxy_index.iter() {
                sum += Self::get_smallest_distance(start, *end);
            }
        }
        sum
    }

    pub fn get_smallest_distance((x0, y0): (usize, usize), (x1, y1): (usize, usize)) -> usize {
        (x0 as isize).abs_diff(x1 as isize) + (y0 as isize).abs_diff(y1 as isize)
    }

    // Debug
    #[allow(dead_code)]
    fn print(&self) {
        for row in self.objects.iter() {
            for object in row.iter() {
                print!("{:?}", object);
            }
            println!();
        }
    }
    #[allow(dead_code)]
    pub fn string_objects(&self) -> String {
        let mut string = String::new();
        for row in self.objects.iter() {
            for object in row.iter() {
                string.push_str(&format!("{:?}", object));
            }
            string.push('\n');
        }
        string
    }
    #[allow(dead_code)]
    pub fn objects(&self) -> &Vec<Vec<SpaceObject>> {
        &self.objects
    }
}

// Parser

#[derive(Default)]
pub enum SpaceObject {
    #[default]
    Empty,
    Galaxy,
}

impl fmt::Debug for SpaceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpaceObject::Empty => write!(f, "."),
            SpaceObject::Galaxy => write!(f, "#"),
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<SpaceObject>> {
    let (input, objects) = many1(alt((
        map(char('.'), |_| SpaceObject::Empty),
        map(char('#'), |_| SpaceObject::Galaxy),
    )))(input)?;

    let (input, _) = alt((tag("\r\n"), tag("\n")))(input)?;

    Ok((input, objects))
}

pub fn parse_galaxy(input: &str) -> IResult<&str, Vec<Vec<SpaceObject>>> {
    let (input, objects) = many1(parse_line)(input)?;

    Ok((input, objects))
}

// Tests
#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let output = solve(input);
        assert_eq!(output, "374".to_string());
    }

    #[test]
    fn path_test() {
        use super::*;

        assert_eq!(Space::get_smallest_distance((0, 0), (0, 0)), 0);
        assert_eq!(Space::get_smallest_distance((5, 0), (9, 11)), 15);
        assert_eq!(Space::get_smallest_distance((0, 2), (12, 7)), 17);
        assert_eq!(Space::get_smallest_distance((0, 0), (5, 0)), 5);
    }

    #[test]
    fn expand_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

        let mut space = super::Space::from(input);
        space.cosmic_expansion();
        assert_eq!(expected, space.string_objects());
    }
}

