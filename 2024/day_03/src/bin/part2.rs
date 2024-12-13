use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for capture in re.captures_iter(input) {
        if &capture[0] == "do()" {
            enabled = true;
        } else if &capture[0] == "don't()" {
            enabled = false;
        } else if enabled {
            let a: usize = capture[1].parse().unwrap();
            let b: usize = capture[2].parse().unwrap();
            sum += a * b;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(part2(input), 48);
    }
}
