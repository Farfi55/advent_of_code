use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let (order_rules, updates) = parse_input(input);

    let mut comes_before_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (before, after) in order_rules {
        let entry = comes_before_map.entry(before).or_insert(HashSet::new());
        entry.insert(after);
    }

    let mut sum = 0;

    for update in updates {
        let mut nums = HashSet::<usize>::new();

        let mut is_valid = true;
        for x in update.iter() {
            if let Some(before) = comes_before_map.get(x) {
                if !nums.is_disjoint(before) {
                    is_valid = false;
                    break;
                }
            }
            nums.insert(*x);
        }

        if is_valid {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let mut order_rules: Vec<(usize, usize)> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let order_parts: Vec<usize> = line.trim().split("|").map(|x| x.parse().unwrap()).collect();
        let x: usize = order_parts[0];
        let y: usize = order_parts[1];

        order_rules.push((x, y));
    }

    while let Some(line) = lines.next() {
        let update_seq = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
        updates.push(update_seq);
    }

    (order_rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(part1(input), 143);
    }
}
