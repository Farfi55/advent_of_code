use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);

    let elapsed_time = now.elapsed();
    println!("main took {} seconds.", elapsed_time.as_secs());
}

fn part1(_input: &str) -> usize {
    let (directions, map) = parse_input(_input);
    follow_directions(&directions, &map)
}

fn follow_directions(directions: &[Direction], map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let next_nodes = map.get(current).unwrap();
        let next = match directions[steps % directions.len()] {
            Direction::Left => next_nodes[0],
            Direction::Right => next_nodes[1],
        };
        current = next;
        steps += 1;
    }
    steps
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, Vec<&str>>) {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines.skip(1) {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap().trim();
        let value = parts
            .next()
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ")
            .collect::<Vec<_>>();
        map.insert(key, value);
    }

    (directions, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const EXAMPLE_INPUT_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn it_follows_example_1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 2);
    }

    fn it_follows_example_2() {
        assert_eq!(part1(EXAMPLE_INPUT_2), 6);
    }

    #[test]
    fn parsing_example_1() {
        let (directions, map) = parse_input(EXAMPLE_INPUT_1);
        assert_eq!(directions, vec![Direction::Right, Direction::Left]);

        assert_eq!(map.len(), 7);
        assert_eq!(map.get("AAA"), Some(&vec!["BBB", "CCC"]));
        assert_eq!(map.get("BBB"), Some(&vec!["DDD", "EEE"]));
        assert_eq!(map.get("CCC"), Some(&vec!["ZZZ", "GGG"]));
        assert_eq!(map.get("DDD"), Some(&vec!["DDD", "DDD"]));
        assert_eq!(map.get("EEE"), Some(&vec!["EEE", "EEE"]));
        assert_eq!(map.get("GGG"), Some(&vec!["GGG", "GGG"]));
        assert_eq!(map.get("ZZZ"), Some(&vec!["ZZZ", "ZZZ"]));
    }

    #[test]
    fn parsing_example_2() {
        let (directions, map) = parse_input(EXAMPLE_INPUT_2);
        assert_eq!(
            directions,
            vec![Direction::Left, Direction::Left, Direction::Right]
        );

        assert_eq!(map.len(), 3);
        assert_eq!(map.get("AAA"), Some(&vec!["BBB", "BBB"]));
        assert_eq!(map.get("BBB"), Some(&vec!["AAA", "ZZZ"]));
        assert_eq!(map.get("ZZZ"), Some(&vec!["ZZZ", "ZZZ"]));
    }
}
