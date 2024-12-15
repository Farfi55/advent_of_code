fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.len()
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = vec![];

    for stone in stones {
        let stone_str = stone.to_string();
        if stone == 0 {
            new_stones.push(1);
        } else if stone_str.len() % 2 == 0 {
            let (left, right) = stone_str.split_at(stone_str.len() / 2);
            new_stones.push(left.parse::<usize>().unwrap());
            new_stones.push(right.parse::<usize>().unwrap());
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"125 17"#;
        assert_eq!(part1(input), 55312);
    }
}
