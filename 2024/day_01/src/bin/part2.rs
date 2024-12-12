use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    let mut left_numbers = Vec::new();
    let mut right_numbers: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let mut numbers: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        left_numbers.push(numbers[0]);

        let r_number = numbers[1];
        let count: &usize = right_numbers.get(&r_number).unwrap_or(&0);
        right_numbers.insert(
            r_number, count + 1);
    }

    left_numbers.sort();

    for num in left_numbers {
        let count = right_numbers.get(&num).unwrap_or(&0);
        result += num * count;
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
3   3"#;
        assert_eq!(part2(input), 31);
    }
}
