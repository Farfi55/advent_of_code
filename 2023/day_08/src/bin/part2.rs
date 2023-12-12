use num::integer::lcm;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);

    let elapsed_time = now.elapsed();
    println!("main took {} seconds.", elapsed_time.as_secs());
}

fn part2(_input: &str) -> usize {
    let (directions, map) = parse_input(_input);
    follow_directions(&directions, &map)
}

fn follow_directions(directions: &[Direction], map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut current_nodes = get_starting_nodes(map);
    let mut steps_to_end_node: Vec<usize> = Vec::new();
    for node in current_nodes.iter_mut() {
        let mut steps = 0;
        while !node.ends_with("Z") {
            let next_nodes = map.get(node).unwrap();
            let next = match directions[steps % directions.len()] {
                Direction::Left => next_nodes[0],
                Direction::Right => next_nodes[1],
            };
            *node = next;
            steps += 1;
        }
        steps_to_end_node.push(steps);
    }
    steps_to_end_node.iter().fold(1, |acc, x| lcm(acc, *x))
}

fn get_starting_nodes<'a>(map: &'a HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    return map
        .keys()
        .into_iter()
        .filter(|k| k.ends_with("A"))
        .map(|k| *k)
        .collect();
}

fn are_all_ending_nodes(nodes: &Vec<&str>) -> bool {
    nodes.iter().all(|n| n.ends_with("Z"))
}

fn is_ending_node(node: &str) -> bool {
    node.ends_with("Z")
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

    const EXAMPLE_INPUT_1: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn it_follows_example_1() {
        assert_eq!(part2(EXAMPLE_INPUT_1), 6);
    }

    #[test]
    fn parsing_example_1() {
        let (directions, map) = parse_input(EXAMPLE_INPUT_1);
        assert_eq!(directions, vec![Direction::Left, Direction::Right]);
        assert_eq!(map.get("11A").unwrap(), &vec!["11B", "XXX"]);
        assert_eq!(map.get("11B").unwrap(), &vec!["XXX", "11Z"]);
        assert_eq!(map.get("11Z").unwrap(), &vec!["11B", "XXX"]);
        assert_eq!(map.get("22A").unwrap(), &vec!["22B", "XXX"]);
        assert_eq!(map.get("22B").unwrap(), &vec!["22C", "22C"]);
        assert_eq!(map.get("22C").unwrap(), &vec!["22Z", "22Z"]);
        assert_eq!(map.get("22Z").unwrap(), &vec!["22B", "22B"]);
        assert_eq!(map.get("XXX").unwrap(), &vec!["XXX", "XXX"]);
    }

    #[test]
    fn test_assign_on_mut_iter() {
        let mut v = vec!['A', 'B', 'C'];
        for i in v.iter_mut() {
            *i = 'Z';
        }
        assert_eq!(v, vec!['Z', 'Z', 'Z']);
    }
}
