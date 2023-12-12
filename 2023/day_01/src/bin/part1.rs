fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("sum:\n{}", result);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_numeric()).collect::<String>();
            let first_digit = digits.chars().next().unwrap();
            let last_digit = digits.chars().last().unwrap();
            let final_number = format!("{}{}", first_digit, last_digit);
            print!("{} ", final_number);

            final_number.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let result = super::part1(input);
        assert_eq!(result, 142);
    }
}
