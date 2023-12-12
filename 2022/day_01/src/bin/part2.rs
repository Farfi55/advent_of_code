fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("sum:\n{}", result);
}

fn part2(input: &str) -> u32 {
    let mut list = Vec::<u32>::new();

    let mut curr_sum = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(num) => curr_sum += num,
            Err(_) => {
                list.push(curr_sum);
                curr_sum = 0;
            }
        }
    }
    list.push(curr_sum);

    list.sort_by(|a, b| b.cmp(a));
    list[0..3].iter().sum()
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

        let result = super::part2(input);
        assert_eq!(result, 45000);
    }
}
