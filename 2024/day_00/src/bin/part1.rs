fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#""#;
        assert_eq!(part1(input), 0);
    }
}
