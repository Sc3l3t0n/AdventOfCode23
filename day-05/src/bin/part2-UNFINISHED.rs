fn main() {
    println!("Part 2");
    let input = include_str!("../../input2.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn split_left(&mut self, at: u64) -> Range {
        let range = Range {
            start: self.start,
            end: at - 1,
        };
        self.start = at;
        range
    }
    fn split_right(&mut self, at: u64) -> Range {
        let range = Range {
            start: at + 1,
            end: self.end,
        };
        self.end = at;
        range
    }
    fn intercept(&mut self, from: u64, to: u64) -> (Range, Range) {
        let left = Range {
            start: self.start,
            end: from - 1,
        };
        let right = Range {
            start: to + 1,
            end: self.end,
        };
        self.start = from;
        self.end = to;
        (left, right)
    }
    fn len(&self) -> u64 {
        self.end - self.start
    }
}

struct ConversionTable {
    destination: Range,
    source: Range,
}

impl From<&str> for ConversionTable {
    fn from(value: &str) -> Self {
        let numbers: Vec<u64> = value
            .split_whitespace()
            .map(|x| x.parse().expect("Should be in right format"))
            .collect();
        Self {
            destination: Range {
                start: numbers[0],
                end: numbers[0] + numbers[2],
            },
            source: Range {
                start: numbers[1],
                end: numbers[1] + numbers[2],
            },
        }
    }
}

impl ConversionTable {
    fn convert(&self, range: &Range) -> Range {
        let offset = range.start - self.source.start;
        let start = self.destination.start + offset;
        Range {
            start,
            end: start + range.len(),
        }
    }
}

fn solve(input: &str) -> String {
    let binding = split_blocks(input);
    let tables: Vec<&str> = binding.iter().map(|x| x.as_str()).collect();

    let mut seeds = parse_seeds(tables[0]);

    let mut seed_to_soil = find_in_ranges(&mut seeds, &parse_conversion_tables(tables[1]));

    let mut soil_to_fertilizer =
        find_in_ranges(&mut seed_to_soil, &parse_conversion_tables(tables[2]));

    let mut fertilizer_to_water =
        find_in_ranges(&mut soil_to_fertilizer, &parse_conversion_tables(tables[3]));

    let mut water_to_light = find_in_ranges(
        &mut fertilizer_to_water,
        &parse_conversion_tables(tables[4]),
    );

    let mut light_to_temperature =
        find_in_ranges(&mut water_to_light, &parse_conversion_tables(tables[5]));

    let mut temperature_to_humidity = find_in_ranges(
        &mut light_to_temperature,
        &parse_conversion_tables(tables[6]),
    );

    let mut humidity_to_location = find_in_ranges(
        &mut temperature_to_humidity,
        &parse_conversion_tables(tables[7]),
    );

    find_lowest(&mut humidity_to_location).to_string()
}

fn split_blocks(input: &str) -> Vec<String> {
    let normalized_input = input.replace("\r\n", "\n");
    normalized_input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

fn parse_seeds(input: &str) -> Vec<Range> {
    let mut lines = input.split(':').skip(1);
    let mut result = Vec::new();
    let seeds = lines
        .next()
        .expect(&format!("Should be in right format ({})", input)[..]);
    let mut iter = seeds.trim().split_whitespace();
    while let (Some(seed_start), Some(seed_range)) = (iter.next(), iter.next()) {
        let seed_start: u64 = seed_start
            .parse()
            .expect(&format!("Should be in right format ({})", input)[..]);
        let seed_range: u64 = seed_range
            .parse()
            .expect(&format!("Should be in right format ({})", input)[..]);

        result.push(Range {
            start: seed_start,
            end: seed_start + seed_range,
        });
    }
    result
}

fn parse_conversion_tables(input: &str) -> Vec<ConversionTable> {
    let mut lines = input.lines();
    let mut result = Vec::new();
    lines.next();
    for line in lines {
        result.push(ConversionTable::from(line));
    }
    result
}

fn find_in_ranges(input: &mut Vec<Range>, ranges: &Vec<ConversionTable>) -> Vec<Range> {
    let mut result = Vec::new();
    let mut found = None;
    let mut offsprings = Vec::new();
    for range in input {
        for table in ranges {
            if range.start >= table.source.start {
                if range.end <= table.source.end {
                    // In Table
                    found = Some(table.convert(range));
                    break;
                } else if range.start <= table.source.end {
                    let other = range.split_right(table.source.end);
                    offsprings.push(other);
                    found = Some(range.clone());
                    break;
                }
            } else if range.end > table.source.start {
                // Over Left Edge
                let other = range.split_left(table.source.start);
                offsprings.push(other);
                found = Some(table.convert(range));
                break;
            } else if range.start < table.source.start && range.end > table.source.end {
                // Out of range
                let other = range.intercept(table.source.start, table.source.end);
                offsprings.push(other.0);
                offsprings.push(other.1);
                found = Some(table.convert(range));
                break;
            }
        }
        match found {
            Some(found) => result.push(found),
            None => result.push(range.clone()),
        }
        found = None;
    }
    if !offsprings.is_empty() {
        result.append(&mut find_in_ranges(&mut offsprings, ranges));
    }
    result
}

fn find_lowest(input: &mut [Range]) -> u64 {
    input.sort_unstable_by_key(|x| x.start);
    input[0].start
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let output = solve(input);
        assert_eq!(output, "46".to_string());
    }
}

