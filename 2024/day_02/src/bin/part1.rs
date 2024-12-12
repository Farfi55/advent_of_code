fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let levels: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if is_safe(&levels) {
            result += 1;
            print!("SAFE: ");
        } else {
            print!("UNSAFE: ");
        }
        println!("{:?}", levels);
    }
    result
}

fn is_safe(levels: &Vec<isize>) -> bool {
    let start_diff = levels[0] - levels[1];
    if start_diff == 0 {
        print!("initial diff is 0: ");
        return false;
    }

    for i in 1..levels.len() {
        let diff = levels[i - 1] - levels[i];
        if start_diff.signum() != diff.signum() {
            print!("diff sign changed: ");
            return false;
        }
        if diff.abs() > 3 {
            print!("diff is greater than 3: ");
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(part1(input), 2);
    }
}
