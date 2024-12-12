fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in input.lines() {
        let mut numbers: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        left_numbers.push(numbers[0]);
        right_numbers.push(numbers[1]);
    }

    left_numbers.sort();
    right_numbers.sort();

    for i in 0..left_numbers.len() {
        result += (left_numbers[i] as isize - right_numbers[i] as isize).abs() as usize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!(part1(input), 11);
    }
}
