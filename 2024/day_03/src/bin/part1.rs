use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for capture in re.captures_iter(input) {
        let a: usize = capture[1].parse().unwrap();
        let b: usize = capture[2].parse().unwrap();
        sum += a * b;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(part1(input), 161);
    }
}
