use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let stones = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    let mut res = 0;
    for stone in stones {
        res += blink(stone, 75, &mut memo)
    }

    res
}

fn blink(stone: usize, times: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if times == 0 {
        return 1;
    }
    if let Some(&res) = memo.get(&(stone, times)) {
        println!("memo hit: {:?}", (stone, times));
        return res;
    } else {
        println!("memo miss: {:?}", (stone, times));
    }
    let res: usize;
    let stone_str = stone.to_string();
    if stone == 0 {
        res = blink(1, times - 1, memo);
    } else if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        res = blink(left.parse::<usize>().unwrap(), times - 1, memo)
            + blink(right.parse::<usize>().unwrap(), times - 1, memo);
    } else {
        res = blink(stone * 2024, times - 1, memo);
    }
    memo.insert((stone, times), res);
    res
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
