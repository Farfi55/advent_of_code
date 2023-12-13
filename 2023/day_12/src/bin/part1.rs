use std::{path::Display, time::Instant};

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);

    let elapsed_time = now.elapsed();
    print_elapsed_time(elapsed_time);
}

fn print_elapsed_time(elapsed_time: std::time::Duration) {
    if elapsed_time.as_secs() == 0 {
        println!(
            "solving the problem took {} milliseconds.",
            elapsed_time.as_millis()
        );
    } else {
        println!(
            "solving the problem took {} seconds.",
            elapsed_time.as_secs()
        );
    }
}

fn part1(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let mut record = parse_condition_record(line);
            get_possible_arrangements(&mut record)
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Condition {
    Working,
    Broken,
    Unknown,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Condition::Working => '.',
            Condition::Broken => '#',
            Condition::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    broken_groups: Vec<usize>,
}

impl std::fmt::Display for ConditionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let conditions = self
            .conditions
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
        let broken_groups = self
            .broken_groups
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{} {}", conditions, broken_groups)
    }
}

fn parse_condition_record(input: &str) -> ConditionRecord {
    let mut parts = input.split_whitespace();
    let conditions_raw = parts.next().unwrap();
    let broken_groups_raw = parts.next().unwrap();

    let conditions = conditions_raw
        .chars()
        .map(|c| match c {
            '#' => Condition::Broken,
            '.' => Condition::Working,
            '?' => Condition::Unknown,
            _ => panic!("Invalid condition character: {}", c),
        })
        .collect();

    let broken_groups = broken_groups_raw
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    ConditionRecord {
        conditions,
        broken_groups,
    }
}

fn get_possible_arrangements(record: &mut ConditionRecord) -> usize {
    recurse(record, 0)
}

fn recurse(record: &mut ConditionRecord, i: usize) -> usize {
    let mut count = 0;
    if i == record.conditions.len() {
        if is_valid_arrangement(record) {
            println!("Found valid arrangement: {}", record);
            return 1;
        } else {
            return 0;
        }
    }

    let condition = record.conditions.get(i).unwrap();
    match condition {
        Condition::Unknown => {
            // try setting to working
            record.conditions[i] = Condition::Working;
            count += recurse(record, i + 1);

            // try setting to broken
            record.conditions[i] = Condition::Broken;
            count += recurse(record, i + 1);

            // Reset the condition to unknown
            record.conditions[i] = Condition::Unknown;
        }
        _ => {
            // nothing to do, move on
            count += recurse(record, i + 1);
        }
    }

    count
}

fn is_valid_arrangement(record: &ConditionRecord) -> bool {
    let mut groups_found = 0;
    let mut curr_group_size = 0;
    for (i, condition) in record.conditions.iter().enumerate() {
        let curr_group = record.broken_groups.get(groups_found);

        match condition {
            Condition::Unknown => {
                return false;
            }
            Condition::Working => match curr_group {
                Some(group_size) => {
                    if curr_group_size != 0 {
                        if curr_group_size != *group_size {
                            return false;
                        }

                        curr_group_size = 0;
                        groups_found += 1;
                    }
                }
                _ => {}
            },
            Condition::Broken => {
                curr_group_size += 1;
                match curr_group {
                    Some(group_size) => {
                        if curr_group_size > *group_size {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
    }
    if curr_group_size != 0 && groups_found == record.broken_groups.len() - 1 {
        if curr_group_size != *record.broken_groups.last().unwrap() {
            return false;
        }

        curr_group_size = 0;
        groups_found += 1;
    }

    groups_found == record.broken_groups.len() && curr_group_size == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn it_follows_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn parse_condition_record_works() {
        let cases = vec![
            (
                "???.### 1,1,3",
                ConditionRecord {
                    conditions: vec![
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Working,
                        Condition::Broken,
                        Condition::Broken,
                        Condition::Broken,
                    ],
                    broken_groups: vec![1, 1, 3],
                },
            ),
            (
                ".??..??...?##. 1,1,3",
                ConditionRecord {
                    conditions: vec![
                        Condition::Working,
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Working,
                        Condition::Working,
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Working,
                        Condition::Working,
                        Condition::Working,
                        Condition::Unknown,
                        Condition::Broken,
                        Condition::Broken,
                        Condition::Working,
                    ],
                    broken_groups: vec![1, 1, 3],
                },
            ),
        ];
        for (input, expected) in cases {
            let condition_record = parse_condition_record(input);
            assert_eq!(condition_record, expected);
        }
    }

    #[test]
    fn arrangements_count_is_correct() {
        let cases = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];

        for (input, expected) in cases {
            let mut condition_record = parse_condition_record(input);
            assert_eq!(get_possible_arrangements(&mut condition_record), expected);
        }
    }

    #[test]
    fn valid_arrangements() {
        let cases = vec![
            ("#.#.### 1,1,3", true),
            ("##..### 1,1,3", false),
            ("#?#.### 1,1,3", false),
            ("?#?#?#?#?#?#?#? 1,3,1,6", false),
            (".#.###.#.###### 1,3,1,6", true),
        ];

        for (input, expected) in cases {
            println!("{}: ", input);
            let condition_record = parse_condition_record(input);
            assert_eq!(is_valid_arrangement(&condition_record), expected);
        }
    }
}
