fn main() {
    println!("Part 1");
    let input = include_str!("../../input1.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

struct Range {
    destination_start: u64,
    source_start: u64,
    source_end: u64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let numbers: Vec<u64> = value
            .split_whitespace()
            .map(|x| x.parse().expect("Should be in right format"))
            .collect();
        Self {
            destination_start: numbers[0],
            source_start: numbers[1],
            source_end: numbers[1] + numbers[2],
        }
    }
}

fn solve(input: &str) -> String {
    // "\r\n" for Windows
    let binding = split_blocks(input);
    let tables: Vec<&str> = binding.iter().map(|x| x.as_str()).collect();
    let seeds = parse_seeds(tables[0]);
    let seed_to_soil = find_in_ranges(&seeds, &parse_ranges(tables[1]));
    let soil_to_fertilizer = find_in_ranges(&seed_to_soil, &parse_ranges(tables[2]));
    let fertilizer_to_water = find_in_ranges(&soil_to_fertilizer, &parse_ranges(tables[3]));
    let water_to_light = find_in_ranges(&fertilizer_to_water, &parse_ranges(tables[4]));
    let light_to_temperature = find_in_ranges(&water_to_light, &parse_ranges(tables[5]));
    let temperature_to_humidity = find_in_ranges(&light_to_temperature, &parse_ranges(tables[6]));
    let mut humidity_to_location =
        find_in_ranges(&temperature_to_humidity, &parse_ranges(tables[7]));
    find_lowest(&mut humidity_to_location).to_string()
}

fn split_blocks(input: &str) -> Vec<String> {
    let normalized_input = input.replace("\r\n", "\n");
    normalized_input.split("\n\n").map(|s| s.to_string()).collect()
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let mut lines = input.split(':').skip(1);
    let mut result = Vec::new();
    let seeds = lines
        .next()
        .expect(&format!("Should be in right format ({})", input)[..]);
    for seed in seeds.trim().split_whitespace() {
        result.push(
            seed.parse()
                .expect(&format!("Should be in right format ({})", input)[..]),
        );
    }
    result
}

fn parse_ranges(input: &str) -> Vec<Range> {
    let mut lines = input.lines();
    let mut result = Vec::new();
    lines.next();
    for line in lines {
        result.push(Range::from(line));
    }
    result
}

fn find_in_ranges(input: &Vec<u64>, ranges: &Vec<Range>) -> Vec<u64> {
    let mut result = Vec::new();
    let mut found = None;
    for source in input {
        for range in ranges {
            if *source >= range.source_start && *source <= range.source_end {
                found = Some(range.destination_start + (source - range.source_start));
                break;
            }
        }
        match found {
            Some(found) => result.push(found),
            None => result.push(*source),
        }
        found = None;
    }
    result
}

fn find_lowest(input: &mut [u64]) -> u64 {
    input.sort_unstable();
    input[0]
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
68 64 1

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let output = solve(input);
        assert_eq!(output, "35".to_string());
    }
}

