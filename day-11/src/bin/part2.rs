use std::collections::HashSet;
use std::fmt;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::map, multi::many1,
    IResult,
};

fn main() {
    println!("Part 2");
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
    expansion_rows: Vec<usize>,
    expansion_cols: Vec<usize>,
    enlargement: usize,
}

impl From<&str> for Space {
    fn from(input: &str) -> Self {
        let (_, objects) = parse_galaxy(input).unwrap();
        Self {
            objects,
            expansion_rows: Vec::new(),
            expansion_cols: Vec::new(),
            enlargement: 1_000_000,
        }
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
        self.expansion_rows.append(&mut rows);
        self.expansion_cols.append(&mut cols);
    }

    fn get_sum_smallest_distance(&self) -> usize {
        let mut sum = 0;
        let mut galaxy_index = self.gen_galaxy_index();
        loop {
            if galaxy_index.is_empty() {
                break;
            }
            let start = galaxy_index.pop().unwrap();
            for end in galaxy_index.iter() {
                sum += self.get_sum_smallest_distance_alt(start, *end);
            }
        }
        sum
    }

    fn get_smallest_distance(&self, (x0, y0): (usize, usize), (x1, y1): (usize, usize)) -> usize {
        let x_range = if x0 > x1 { x1..x0 } else { x0..x1 };
        let x_offset = self
            .expansion_rows
            .iter()
            .filter(|&row| x_range.contains(row))
            .count();
        let y_range = if y0 > y1 { y1..y0 } else { y0..y1 };
        let y_offset = self
            .expansion_cols
            .iter()
            .filter(|&col| y_range.contains(col))
            .count();

        (x0 as isize).abs_diff(x1 as isize)
            + (y0 as isize).abs_diff(y1 as isize)
            + ((x_offset + y_offset) * self.enlargement)
    }

    fn get_sum_smallest_distance_alt(
        &self,
        (x0, y0): (usize, usize),
        (x1, y1): (usize, usize),
    ) -> usize {
        let nx0 = x0
            + self.expansion_rows.iter().filter(|&row| *row < x0).count() * (self.enlargement - 1);
        let nx1 = x1
            + self.expansion_rows.iter().filter(|&row| *row < x1).count() * (self.enlargement - 1);
        let ny0 = y0
            + self.expansion_cols.iter().filter(|&col| *col < y0).count() * (self.enlargement - 1);
        let ny1 = y1
            + self.expansion_cols.iter().filter(|&col| *col < y1).count() * (self.enlargement - 1);

        (nx0 as isize).abs_diff(nx1 as isize) + (ny0 as isize).abs_diff(ny1 as isize)
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
    #[allow(dead_code)]
    pub fn set_enlargement(&mut self, enlargement: usize) {
        self.enlargement = enlargement;
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
#[cfg(test)]
mod tests {
    use crate::Space;

    #[test]
    fn input_100() {
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

        let mut space = Space::from(input);
        space.set_enlargement(100);
        space.cosmic_expansion();
        let output = space.get_sum_smallest_distance().to_string();
        assert_eq!(output, "8410".to_string());
    }

    #[test]
    fn input_10() {
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
        let mut space = Space::from(input);
        space.set_enlargement(10);
        space.cosmic_expansion();
        let output = space.get_sum_smallest_distance().to_string();

        assert_eq!(output, "1030".to_string());
    }
}

