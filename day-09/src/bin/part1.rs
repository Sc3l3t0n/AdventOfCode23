fn main() {
    println!("Part 1");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

fn solve(input: &str) -> String {
    let datas = parse(input);
    calculate_all_values(&datas).to_string()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for value in line.split_whitespace() {
            row.push(value.parse().unwrap());
        }
        result.push(row);
    }
    result
}

fn calculate_all_values(values: &[Vec<i32>]) -> i32 {
    let mut result = 0;

    for row in values {
        result += calculate_next_value(row);
    }

    result
}

fn calculate_next_value(values: &[i32]) -> i32 {
    let mut result = 0;
    let mut stages: Vec<Vec<i32>> = vec![values.to_vec()];
    while stages.last().unwrap().iter().any(|&x| x != 0) {
        let stage = stages.last().unwrap();
        let mut new_stage = Vec::new();
        for i in 0..stage.len() - 1 {
            new_stage.push(*stage.get(i + 1).unwrap() - *stage.get(i).unwrap());
        }
        stages.push(new_stage);
    }
    for stage in stages {
        result += stage.last().unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let output = solve(input);
        assert_eq!(output, "114".to_string());
    }
}

