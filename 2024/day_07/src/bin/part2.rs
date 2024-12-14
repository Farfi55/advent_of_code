use itertools::Itertools;
fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let mut line_split = line.split(": ");
        let target: usize = line_split.next().unwrap().parse::<usize>().unwrap();
        let numbers: Vec<usize> = line_split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if exists_valid_equation(target, &numbers) {
            sum += target;
        }
    }

    sum
}

fn exists_valid_equation(target: usize, numbers: &Vec<usize>) -> bool {
    println!("Target: {:14}, Numbers: {:?}", target, numbers);

    let k = numbers.len() - 1;
    for mut combination in 0..3_usize.pow(k as u32) {
        let mut result: usize = numbers[0];
        for i in 0..k {
            match combination % 3 {
                0 => result += numbers[i + 1],
                1 => result *= numbers[i + 1],
                2 => {
                    result = (result.to_string() + &numbers[i + 1].to_string())
                        .parse()
                        .unwrap()
                }
                _ => panic!("Invalid combination"),
            }
            combination /= 3;
        }

        if result == target {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(part2(input), 11387);
    }
}
