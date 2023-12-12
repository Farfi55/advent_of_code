fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("sum:\n{}", result);
}

fn part1(input: &str) -> u32 {
    let mut max_sum = 0;
    let mut curr_sum = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(num) => curr_sum += num,
            Err(_) => {
                if curr_sum > max_sum {
                    max_sum = curr_sum;
                }
                curr_sum = 0;
            }
        }
    }
    if curr_sum > max_sum {
        max_sum = curr_sum;
    }

    max_sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let result = super::part1(input);
        assert_eq!(result, 24000);
    }
}
